import {
  CoreObject,
  Entity,
  EntityReference,
  Link,
} from "activitypub-core-types/lib/activitypub";
import { BaseEntity } from "activitypub-core-types/lib/activitypub/Core/Entity";
import { URL } from "url";
import { ACCEPT_HEADER, USER_AGENT } from "./consts";

export async function resolveApEntity(ref: EntityReference): Promise<Entity> {
  if (typeof ref == "string") {
    return (await getApDocument(ref)) as Entity;
  } else if (typeof (ref as Link).href == "string") {
    return resolveApEntity((ref as Link).href!!);
  } else if (typeof (ref as CoreObject).type == "string") {
    return ref as CoreObject;
  } else {
    throw `${ref} cannot be resolved as a AP entity`;
  }
}

export async function getApDocument(url: URL): Promise<BaseEntity> {
  console.log(`resolving AP document ${url}, UA: ${USER_AGENT}`);
  const result = await fetch(url, {
    method: "GET",
    headers: {
      "User-Agent": USER_AGENT,
      Accept: ACCEPT_HEADER,
    },
  });
  console.log(`resolved AP doc ${url}: ${result.status}`);
  const json = await result.text();
  try {
    return JSON.parse(json) as BaseEntity;
  } catch (e) {
    console.error(json);
    throw e;
  }
}
