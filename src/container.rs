use std::{cmp, collections::HashMap, env};

use anyhow::{Context, Result};
use podman_api::{
    api::Containers,
    models::*,
    opts::{
        ContainerCreateOpts, ContainerCreateOptsBuilder, ContainerDeleteOpts, ContainerListOpts,
        ContainerRestartPolicy, ImageVolumeMode, SeccompPolicy, SocketNotifyMode, SystemdEnabled,
    },
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{constant::ENV_PURGE_RUNNING_CTRS, direct_into_build};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct ContainerResources {
    #[serde(default)]
    pub created: Vec<ContainerCreated>,
    #[serde(default)]
    pub removed: Vec<ContainerRemoved>,
}

impl ContainerResources {
    pub async fn apply(&self, api: &Containers) -> Result<()> {
        for created in &self.created {
            let remote = api.get(&created.name);
            if remote.exists().await? {
                if created == &remote.inspect().await? {
                    continue;
                } else {
                    remote.delete(&Default::default()).await?;
                    info!(
                        name = created.name,
                        "deleted exists container for not matching"
                    )
                }
            }
            let resp = api
                .create(&created.clone().into())
                .await
                .with_context(|| format!("target container: {}", &created.name))?;
            info!(
                name = created.name,
                response = serde_json::to_string(&resp)?,
                "created container"
            );
        }
        for removed in &self.removed {
            let remote = api.get(&removed.name);
            if remote.exists().await? {
                let force = removed.force.unwrap_or(false);
                if force {
                    remote.remove().await?;
                } else {
                    remote
                        .delete(&ContainerDeleteOpts::builder().force(true).build())
                        .await?;
                }
                info!(name = removed.name, force, "deleted exists container");
            }
        }
        Ok(())
    }

    pub async fn purge(&self, api: &Containers) -> Result<()> {
        // todo: skip podrc container
        // todo: https://github.com/containers/podman/pull/19911
        let container = api
            .list(
                &ContainerListOpts::builder()
                    /* .filter([ContainerListFilter::NoLabelKeyVal(
                        LABEL_NO_PURGE.to_string(),
                        LABEL_NO_PURGE_VAL.to_string(),
                    )])*/
                    .build(),
            )
            .await?;
        let managed = self
            .created
            .iter()
            .map(|f| f.name.to_owned())
            .collect::<Vec<_>>();
        for container in container {
            let id = container.id.unwrap();
            if !managed.contains(&id)
                && !container
                    .names
                    .unwrap_or_default()
                    .iter()
                    .any(|f| managed.contains(f))
            {
                let remote = api.get(&id);
                if remote
                    .inspect()
                    .await?
                    .state
                    .and_then(|f| f.running)
                    .unwrap_or_default()
                    && env::var(ENV_PURGE_RUNNING_CTRS).unwrap() != "true"
                {
                    continue;
                }
                remote.delete(&Default::default()).await?;
                info!(id, "purged container");
            }
        }
        Ok(())
    }

    pub fn merge(self, new: &mut Self) {
        new.created.extend(self.created);
        new.removed.extend(self.removed);
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct ContainerCreated {
    #[serde(default)]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(default)]
    pub apparmor_profile: String,
    #[serde(default)]
    pub add_caps: Vec<String>,
    #[serde(default)]
    pub drop_caps: Vec<String>,
    #[serde(default)]
    pub cgroup_parent: Option<String>,
    #[serde(default)]
    pub cgroup_ns: Option<Namespace>,
    #[serde(default)]
    pub cgroup_mode: Option<String>,
    #[serde(default)]
    pub chroot_dirs: Vec<String>,
    #[serde(default)]
    pub cmd: Option<Vec<String>>,
    #[serde(default)]
    pub common_pid_file: Option<String>,
    #[serde(default)]
    pub cpu_period: u64,
    #[serde(default)]
    pub cpu_quota: i64,
    #[serde(default = "value_true")]
    pub create_working_dir: bool,
    #[serde(default)]
    pub dependency_containers: Vec<String>,
    #[serde(default)]
    pub device_cgroup_rule: Option<Vec<LinuxDeviceCgroup>>,
    #[serde(default)]
    pub devices: Vec<LinuxDevice>,
    #[serde(default)]
    pub dns_option: Vec<String>,
    #[serde(default)]
    pub dns_search: Vec<String>,
    #[serde(default)]
    pub dns_server: Vec<String>,
    #[serde(default)]
    pub entrypoint: Option<Vec<String>>,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub env_host: bool,
    #[serde(default)]
    pub envmerge: Vec<String>,
    #[serde(default)]
    pub groups: Vec<String>,
    #[serde(default)]
    pub health_check_on_failure_action: Option<i64>,
    #[serde(default)]
    pub health_config: Option<Schema2HealthConfig>,
    #[serde(default)]
    pub host_device_list: Vec<LinuxDevice>,
    #[serde(default)]
    pub hosts_add: Vec<String>,
    #[serde(default)]
    pub hostname: Option<String>,
    #[serde(default)]
    pub hostusers: Vec<String>,
    #[serde(default = "value_true")]
    pub http_proxy: bool,
    #[serde(default)]
    pub id_mappings: Option<IdMappingOptions>,
    pub image: String,
    #[serde(default)]
    pub image_arch: Option<String>,
    #[serde(default)]
    pub image_os: Option<String>,
    #[serde(default)]
    pub image_variant: Option<String>,
    #[serde(default)]
    pub image_volume_mode: Option<ImageVolumeMode>,
    #[serde(default)]
    pub image_volumes: Option<Vec<ImageVolume>>,
    #[serde(default)]
    pub init: bool,
    #[serde(default)]
    pub init_container_type: Option<String>,
    #[serde(default)]
    pub init_path: Option<String>,
    #[serde(default)]
    pub ipc_namespace: Option<Namespace>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub log_configuration: Option<LogConfig>,
    #[serde(default)]
    pub mask: Option<Vec<String>>,
    #[serde(default)]
    pub mounts: Vec<ContainerMount>,
    pub name: String,
    #[serde(default)]
    pub namespace: String,
    #[serde(default)]
    pub net_namespace: Option<Namespace>,
    #[serde(default)]
    pub network_options: Option<HashMap<String, String>>,
    #[serde(default)]
    pub networks: HashMap<String, String>,
    #[serde(default)]
    pub no_new_privilages: bool,
    #[serde(default)]
    pub oci_runtime: Option<String>,
    #[serde(default)]
    pub oom_score_adj: Option<i64>,
    #[serde(default)]
    pub overlay_volumes: Vec<OverlayVolume>,
    #[serde(default)]
    pub passwd_entry: Option<String>,
    #[serde(default)]
    pub personality: Option<LinuxPersonality>,
    #[serde(default)]
    pub pid_namespace: Option<Namespace>,
    #[serde(default)]
    pub pod: Option<String>,
    #[serde(default)]
    pub portmappings: Vec<PortMapping>,
    #[serde(default)]
    pub privileged: bool,
    #[serde(default)]
    pub procfs_opts: Option<Vec<String>>,
    #[serde(default)]
    pub publish_image_ports: bool,
    #[serde(default)]
    pub r_limits: Vec<PosixRlimit>,
    #[serde(default)]
    pub raw_image_name: Option<String>,
    #[serde(default)]
    pub read_only_fs: bool,
    #[serde(default)]
    pub remove: bool,
    #[serde(default)]
    pub resource_limits: Option<LinuxResources>,
    #[serde(default)]
    pub restart_policy: Option<ContainerRestartPolicy>,
    #[serde(default)]
    pub restart_tries: u64,
    #[serde(default)]
    pub rootfs: String,
    #[serde(default)]
    pub rootfs_overlay: bool,
    #[serde(default)]
    pub rootfs_propagation: Option<String>,
    #[serde(default)]
    pub sdnotify_mode: Option<SocketNotifyMode>,
    #[serde(default)]
    pub seccomp_policy: Option<SeccompPolicy>,
    #[serde(default)]
    pub seccomp_profile_path: Option<String>,
    #[serde(default)]
    pub secret_env: HashMap<String, String>,
    #[serde(default)]
    pub secrets: Vec<Secret>,
    #[serde(default)]
    pub selinux_opts: Vec<String>,
    #[serde(default)]
    pub shm_size: Option<i64>,
    #[serde(default)]
    pub stdin: bool,
    #[serde(default)]
    pub stop_signal: Option<i64>,
    #[serde(default)]
    pub stop_timeout: Option<u64>,
    #[serde(default)]
    pub storage_opts: HashMap<String, String>,
    #[serde(default)]
    pub sysctl: HashMap<String, String>,
    #[serde(default)]
    pub systemd: Option<SystemdEnabled>,
    #[serde(default)]
    pub terminal: bool,
    #[serde(default)]
    pub throttle_read_bps_device: HashMap<String, String>,
    #[serde(default)]
    pub throttle_read_iops_device: HashMap<String, String>,
    #[serde(default)]
    pub throttle_write_bps_device: HashMap<String, String>,
    #[serde(default)]
    pub throttle_write_iops_device: HashMap<String, String>,
    #[serde(default)]
    pub timeout: u64,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub umask: Option<String>,
    #[serde(default)]
    pub unified: HashMap<String, String>,
    #[serde(default)]
    pub unmask: Vec<String>,
    #[serde(default)]
    pub unset_env: Vec<String>,
    #[serde(default)]
    pub unset_env_all: bool,
    #[serde(default)]
    pub use_image_hosts: bool,
    #[serde(default)]
    pub use_image_resolv_conf: bool,
    #[serde(default)]
    pub user: Option<String>,
    #[serde(default)]
    pub user_namespace: Option<Namespace>,
    #[serde(default)]
    pub uts_namespace: Option<Namespace>,
    #[serde(default)]
    pub volatile: bool,
    #[serde(default)]
    pub volumes: Vec<NamedVolume>,
    #[serde(default)]
    pub weight_device: Option<LinuxWeightDevice>,
    #[serde(default)]
    pub work_dir: Option<String>,
}

impl Eq for ContainerCreated {}

fn value_true() -> bool {
    true
}

impl From<ContainerCreated> for ContainerCreateOptsBuilder {
    fn from(val: ContainerCreated) -> Self {
        let mut builder = ContainerCreateOpts::builder()
            .apparmor_profile(val.apparmor_profile)
            .add_capabilities(val.add_caps)
            .drop_capabilities(val.drop_caps)
            .chroot_directories(val.chroot_dirs)
            .cpu_period(val.cpu_period)
            .cpu_quota(val.cpu_quota)
            .create_working_dir(val.create_working_dir)
            .dependency_containers(val.dependency_containers)
            .devices(val.devices)
            .dns_option(val.dns_option)
            .dns_search(val.dns_search)
            .dns_server(val.dns_server)
            .env(val.env)
            .env_host(val.env_host)
            .envmerge(val.envmerge)
            .groups(val.groups)
            .host_device_list(val.host_device_list)
            .hosts_add(val.hosts_add)
            .hostusers(val.hostusers)
            .http_proxy(val.http_proxy)
            .image(val.image)
            .init(val.init)
            .labels(val.labels)
            .mounts(val.mounts)
            .name(val.name)
            .namespace(val.namespace)
            .networks(val.networks)
            .no_new_privilages(val.no_new_privilages)
            .overlay_volumes(val.overlay_volumes)
            .portmappings(val.portmappings)
            .privileged(val.privileged)
            .publish_image_ports(val.publish_image_ports)
            .r_limits(val.r_limits)
            .read_only_fs(val.read_only_fs)
            .remove(val.remove)
            .restart_tries(val.restart_tries)
            .rootfs(val.rootfs)
            .rootfs_overlay(val.rootfs_overlay)
            .secret_env(val.secret_env)
            .secrets(val.secrets)
            .selinux_opts(val.selinux_opts)
            .stdin(val.stdin)
            .storage_opts(val.storage_opts)
            .sysctl(val.sysctl)
            .terminal(val.terminal)
            .throttle_read_bps_device(val.throttle_read_bps_device)
            .throttle_read_iops_device(val.throttle_read_iops_device)
            .throttle_write_bps_device(val.throttle_write_bps_device)
            .throttle_write_iops_device(val.throttle_write_iops_device)
            .timeout(val.timeout)
            .unified(val.unified)
            .unmask(val.unmask)
            .unset_env(val.unset_env)
            .unset_env_all(val.unset_env_all)
            .use_image_hosts(val.use_image_hosts)
            .use_image_resolv_conf(val.use_image_resolv_conf)
            .volatile(val.volatile)
            .volumes(val.volumes);
        if let Some(value) = val.annotations {
            builder = builder.annotations(value);
        }
        if let Some(value) = val.cgroup_parent {
            builder = builder.cgroup_parent(value);
        }
        if let Some(value) = val.cgroup_ns {
            builder = builder.cgroup_namespace(value);
        }
        if let Some(value) = val.cgroup_mode {
            builder = builder.cgroup_mode(value);
        }
        if let Some(value) = val.cmd {
            builder = builder.command(value);
        }
        if let Some(value) = val.common_pid_file {
            builder = builder.common_pid_file(value);
        }
        if let Some(value) = val.device_cgroup_rule {
            builder = builder.device_cgroup_rule(value);
        }
        if let Some(value) = val.entrypoint {
            builder = builder.entrypoint(value);
        }
        if let Some(value) = val.health_check_on_failure_action {
            builder = builder.health_check_on_failure_action(value);
        }
        if let Some(value) = val.health_config {
            builder = builder.health_config(value);
        }
        if let Some(value) = val.hostname {
            builder = builder.hostname(value);
        }
        if let Some(value) = val.id_mappings {
            builder = builder.id_mappings(value);
        }
        if let Some(value) = val.image_arch {
            builder = builder.image_arch(value);
        }
        if let Some(value) = val.image_os {
            builder = builder.image_os(value);
        }
        if let Some(value) = val.image_variant {
            builder = builder.image_variant(value);
        }
        if let Some(value) = val.image_volume_mode {
            builder = builder.image_volume_mode(value);
        }
        if let Some(value) = val.image_volumes {
            builder = builder.image_volumes(value);
        }
        if let Some(value) = val.init_container_type {
            builder = builder.init_container_type(value);
        }
        if let Some(value) = val.init_path {
            builder = builder.init_path(value);
        }
        if let Some(value) = val.ipc_namespace {
            builder = builder.ipc_namespace(value);
        }
        if let Some(value) = val.log_configuration {
            builder = builder.log_configuration(value);
        }
        if let Some(value) = val.mask {
            builder = builder.mask(value);
        }
        if let Some(value) = val.net_namespace {
            builder = builder.net_namespace(value);
        }
        if let Some(value) = val.network_options {
            builder = builder.network_options(value);
        }
        if let Some(value) = val.oci_runtime {
            builder = builder.oci_runtime(value);
        }
        if let Some(value) = val.oom_score_adj {
            builder = builder.oom_score_adj(value);
        }
        if let Some(value) = val.passwd_entry {
            builder = builder.passwd_entry(value);
        }
        if let Some(value) = val.personality {
            builder = builder.personality(value);
        }
        if let Some(value) = val.pid_namespace {
            builder = builder.pid_namespace(value);
        }
        if let Some(value) = val.pod {
            builder = builder.pod(value);
        }
        if let Some(value) = val.procfs_opts {
            builder = builder.procfs_opts(value);
        }
        if let Some(value) = val.raw_image_name {
            builder = builder.raw_image_name(value);
        }
        if let Some(value) = val.resource_limits {
            builder = builder.resource_limits(value);
        }
        if let Some(value) = val.restart_policy {
            builder = builder.restart_policy(value);
        }
        if let Some(value) = val.rootfs_propagation {
            builder = builder.rootfs_propagation(value);
        }
        if let Some(value) = val.sdnotify_mode {
            builder = builder.sdnotify_mode(value);
        }
        if let Some(value) = val.seccomp_policy {
            builder = builder.seccomp_policy(value);
        }
        if let Some(value) = val.seccomp_profile_path {
            builder = builder.seccomp_profile_path(value);
        }
        if let Some(value) = val.shm_size {
            builder = builder.shm_size(value);
        }
        if let Some(value) = val.stop_signal {
            builder = builder.stop_signal(value);
        }
        if let Some(value) = val.stop_timeout {
            builder = builder.stop_timeout(value);
        }
        if let Some(value) = val.systemd {
            builder = builder.systemd(value);
        }
        if let Some(value) = val.timezone {
            builder = builder.timezone(value);
        }
        if let Some(value) = val.umask {
            builder = builder.umask(value);
        }
        if let Some(value) = val.user {
            builder = builder.user(value);
        }
        if let Some(value) = val.user_namespace {
            builder = builder.user_namespace(value);
        }
        if let Some(value) = val.uts_namespace {
            builder = builder.uts_namespace(value);
        }
        if let Some(value) = val.weight_device {
            builder = builder.weight_device(value);
        }
        if let Some(value) = val.work_dir {
            builder = builder.work_dir(value);
        }
        builder
    }
}

direct_into_build!(ContainerCreated, ContainerCreateOptsBuilder => ContainerCreateOpts);

impl cmp::PartialEq<InspectContainerData> for ContainerCreated {
    fn eq(&self, other: &InspectContainerData) -> bool {
        let other = other.to_owned();
        let config = other.config.unwrap();
        self.name == other.name.unwrap()
            && self.labels == config.labels.unwrap_or_default()
            && self.apparmor_profile == other.app_armor_profile.unwrap_or_default()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct ContainerRemoved {
    pub name: String,
    #[serde(default)]
    pub force: Option<bool>,
}
