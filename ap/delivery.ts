import { signRequest } from "@peertube/http-signature";
import { BaseEntity } from "activitypub-core-types/lib/activitypub/Core/Entity";
import { ClientRequest } from "http";
import { env } from "process";
import { ACCEPT_HEADER, USER_AGENT } from "./consts";

export async function deliveryAPActivity(url: URL, doc: BaseEntity) {
  console.log(`deliverying AP document ${url}`);

  const request = {
    url: url.toString(),
    method: "POST",
    headers: {
      Date: new Date().toUTCString(),
      Host: url.hostname,
    },
  };
  const signature = signRequest(request as unknown as ClientRequest, {
    key: env["XTEX_HOME_AP_PRIV_KEY"]!!,
    keyId: "XTEX-HOME-AP-INSTANCE-ACTOR",
  });
  console.log(signature);

  const result = await fetch(url, {
    method: "POST",
    body: JSON.stringify(doc),
    headers: {
      "User-Agent": USER_AGENT,
      Accept: ACCEPT_HEADER,
      "Content-Type": "application/activity+json",
    },
  });

  console.log(`delivered AP doc ${url}: ${result.status}`);
}
