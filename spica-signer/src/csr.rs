use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Result};
use openssl::{
    asn1::{Asn1Integer, Asn1Time},
    bn::{BigNum, MsbOption},
    hash::MessageDigest,
    nid::Nid,
    pkey::PKey,
    x509::{X509Name, X509},
};
use pem::Pem;
use picky_asn1_x509::{AttributeValues, CertificationRequest, ExtensionView};
use serde::{Deserialize, Serialize};
use spica_signer_common::req::CSR;

use crate::{
    acl::ACLRule,
    cert::CACert,
    openssl::{get_ossl_conf, OpenSSLOpts, OpenSSLOptsExt, X509NameContainer},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CertReq {
    pub not_before: SystemTime,
    pub not_after: SystemTime,
    pub serial: Option<String>,
    pub subject_name: X509NameContainer,
    #[serde(rename = "openssl-opt", default)]
    pub ossl_opt: OpenSSLOpts,
    pub pubkey_pem: String,
    pub prefer_hash: Option<String>,
}

impl CertReq {
    pub fn from_csr(
        pem: &Pem,
        validity: &Option<(SystemTime, SystemTime)>,
        serial: &Option<String>,
        acl: &ACLRule,
        fallback_prefer_hash: &Option<String>,
    ) -> Result<CertReq> {
        if pem.tag != "CERTIFICATE REQUEST" {
            bail!("unexpected pem tag {}", pem.tag)
        }
        if let Some(_) = serial && !acl.can_custom_serial {
            bail!("custom serial is not allowed by ACL")
        }
        let req = picky_asn1_der::from_bytes::<CertificationRequest>(pem.contents.as_slice())?;
        // let ossl_req = X509Req::from_pem(pem::encode(&pem).as_bytes())?;
        let info = req.certification_request_info;
        let subject_name = X509NameContainer::from_picky(info.subject)?;
        let ossl_spki = PKey::public_key_from_der(
            picky_asn1_der::to_vec(&info.subject_public_key_info)?.as_slice(),
        )?;
        let (not_before, not_after) = validity.unwrap_or_else(|| {
            let now = SystemTime::now();
            (now, now + acl.max_expire)
        });
        let duration = not_after.duration_since(not_before)?;
        if duration > acl.max_expire {
            bail!(
                "valid duration {}s > max ACL duration {}s",
                duration.as_secs(),
                acl.max_expire.as_secs()
            )
        }
        let mut ossl_opt = acl.openssl_opt.to_owned();

        // copy SAN DNS
        let san_matchers = acl.san_dns_to_regexs()?;
        if let Some(regexs) = &san_matchers {
            let mut san_names = Vec::new();
            for attr in info.attributes.iter() {
                if let AttributeValues::Extensions(exts) = &attr.value {
                    for ext_set in exts.iter() {
                        for ext in ext_set.0.iter() {
                            if let ExtensionView::SubjectAltName(names) = ext.extn_value() {
                                for name in names.iter() {
                                    if let picky_asn1_x509::GeneralName::DnsName(name) = name {
                                        let name = name.to_string();
                                        if regexs.is_empty() {
                                            san_names.push(name);
                                        } else {
                                            for regex in regexs.iter() {
                                                if regex.is_match(&name) {
                                                    san_names.push(name);
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !san_names.is_empty() {
                ossl_opt.insert(
                    "subjectAltName".to_owned(),
                    san_names
                        .iter()
                        .map(|v| format!("DNS:{v}"))
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }

            if !regexs.is_empty() {
                // check CN
                for cn in subject_name.0.entries_by_nid(Nid::COMMONNAME) {
                    let cn = cn.data().as_utf8()?.to_string();
                    if !cn.is_empty() {
                        let first = cn.as_bytes()[0];
                        let last = *cn.as_bytes().last().unwrap();
                        if cn.contains('.')
                            && cn
                                .chars()
                                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '/' | '.'))
                            && first.is_ascii_alphanumeric()
                            && (last.is_ascii_alphanumeric() || last == b'*')
                        {
                            let mut matched = false;
                            for regex in regexs.iter() {
                                if regex.is_match(&cn) {
                                    matched = true;
                                    break;
                                }
                            }
                            if !matched {
                                bail!("subject CN {} seems to be a domain name but is forbidden by ACL", cn)
                            }
                        }
                    }
                }
            }
        }

        Ok(CertReq {
            not_before,
            not_after,
            serial: serial.to_owned(),
            subject_name,
            ossl_opt,
            pubkey_pem: String::from_utf8(ossl_spki.public_key_to_pem()?)?,
            prefer_hash: acl
                .prefer_hash
                .to_owned()
                .or(fallback_prefer_hash.to_owned()),
        })
    }

    pub fn from_json(
        csr: &CSR,
        acl: &ACLRule,
        fallback_prefer_hash: &Option<String>,
    ) -> Result<CertReq> {
        if let Some(_) = csr.serial && !acl.can_custom_serial {
            bail!("custom serial is not allowed by ACL")
        }
        if !csr.extra_ossl_opts.is_empty() && !acl.can_custom_openssl_opts {
            bail!("custom OpenSSL options are not allowed by ACL")
        }
        let san_matchers = acl.san_dns_to_regexs()?;

        let pubkey_pem = match &csr.public_key_pem {
            Some(k) => k.to_owned(),
            None => bail!("pubkey_pem is required for signing"),
        };

        // subject name
        let mut subject_name = X509Name::builder()?;
        for (k, v) in &csr.name {
            if k == "CN" && let Some(regexs) = &san_matchers && !regexs.is_empty() && !v.is_empty() {
                let first = v.as_bytes()[0];
                let last = *v.as_bytes().last().unwrap();
                if v.contains('.')
                    && v.chars()
                        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '/' | '.'))
                    && first.is_ascii_alphanumeric()
                    && (last.is_ascii_alphanumeric() || last == b'*')
                {
                    let mut matched = false;
                    for regex in regexs.iter() {
                        if regex.is_match(v) {
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        bail!(
                            "subject CN {} seems to be a domain name but is forbidden by ACL",
                            v
                        )
                    }
                }
            }
            subject_name.append_entry_by_text(k.as_str(), v.as_str())?;
        }

        // validity
        let not_before = csr.not_before.unwrap_or_else(SystemTime::now);
        let not_after = not_before + csr.expiry.unwrap_or(acl.max_expire);

        let duration = not_after.duration_since(not_before)?;
        if duration > acl.max_expire {
            bail!(
                "valid duration {}s > max ACL duration {}s",
                duration.as_secs(),
                acl.max_expire.as_secs()
            )
        }

        let mut ossl_opt = acl.openssl_opt.to_owned();
        ossl_opt.extend(csr.extra_ossl_opts.clone());

        // copy SAN DNS
        if let Some(regexs) = &san_matchers {
            let mut hosts = Vec::new();
            for host in &csr.hosts {
                if regexs.is_empty() {
                    hosts.push(host);
                } else {
                    for regex in regexs.iter() {
                        if regex.is_match(host) {
                            hosts.push(host);
                            break;
                        }
                    }
                }
            }
            if !hosts.is_empty() {
                ossl_opt.insert(
                    "subjectAltName".to_owned(),
                    hosts
                        .iter()
                        .map(|v| format!("DNS:{v}"))
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
        }

        Ok(CertReq {
            not_before,
            not_after,
            serial: csr.serial.to_owned(),
            subject_name: X509NameContainer(subject_name.build()),
            ossl_opt,
            pubkey_pem,
            prefer_hash: acl
                .prefer_hash
                .to_owned()
                .or(fallback_prefer_hash.to_owned()),
        })
    }

    pub fn sign(&self, ca: &CACert) -> Result<String> {
        let mut builder = X509::builder()?;

        builder.set_version(2)?;
        builder.set_not_before(
            Asn1Time::from_unix(self.not_before.duration_since(UNIX_EPOCH)?.as_secs() as i64)?
                .as_ref(),
        )?;
        builder.set_not_after(
            Asn1Time::from_unix(self.not_after.duration_since(UNIX_EPOCH)?.as_secs() as i64)?
                .as_ref(),
        )?;
        let serial = match &self.serial {
            Some(serial) => BigNum::from_hex_str(serial)?,
            None => Self::rand_serial()?,
        };
        builder.set_serial_number(Asn1Integer::from_bn(&serial)?.as_ref())?;
        let ca_crt = ca.to_ossl_x509()?;
        builder.set_issuer_name(ca_crt.subject_name())?;
        builder.set_subject_name(&self.subject_name)?;
        let pubkey = PKey::public_key_from_pem(self.pubkey_pem.as_bytes())?;
        builder.set_pubkey(&pubkey)?;

        let mut ossl_opts = OpenSSLOpts::new();
        ossl_opts.insert(
            "basicConstraints".to_owned(),
            "critical, CA:FALSE".to_string(),
        );
        ossl_opts.extend(ca.config.openssl_opt.to_owned());
        ossl_opts.extend(self.ossl_opt.to_owned());
        let ossl_ctx = builder.x509v3_context(Some(&ca_crt), Some(get_ossl_conf()));
        for ext in ossl_opts.to_exts(&ossl_ctx)?.iter() {
            builder.append_extension2(ext)?;
        }

        let priv_key = ca.to_ossl_pkey()?;
        let hash = self
            .prefer_hash
            .to_owned()
            .and_then(|v| MessageDigest::from_name(v.as_str()))
            .unwrap_or(MessageDigest::sha512());
        builder.sign(&priv_key, hash)?;

        let cert = builder.build();
        Ok(String::from_utf8(cert.to_pem()?)?)
    }

    fn rand_serial() -> Result<BigNum> {
        let mut bn = BigNum::new()?;
        bn.rand(159, MsbOption::MAYBE_ZERO, false)?;
        Ok(bn)
    }
}
