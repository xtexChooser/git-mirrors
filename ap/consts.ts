import { env } from "process";

export const ACCEPT_HEADER =
  'application/ld+json; profile="https://www.w3.org/ns/activitystreams"';
export const USER_AGENT = `xtex-home/1.0@${env["VERCEL_GIT_COMMIT_SHA"]} (${env["NEXT_PUBLIC_VERCEL_URL"]})`;
