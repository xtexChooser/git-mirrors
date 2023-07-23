import * as httpSignature from "@peertube/http-signature";
import {
    Activity,
    ActivityTypes,
    Actor,
    Reject,
} from "activitypub-core-types/lib/activitypub";
import type { NextApiRequest, NextApiResponse } from "next";
import { deliveryAPActivity } from "../../../ap/delivery";
import { resolveApEntity } from "../../../ap/resolver";

export default async function handler(
    req: NextApiRequest,
    res: NextApiResponse
) {
    const signature = req.headers["signature"];
    const activity = req.body as Activity;
    if (signature == null) {
        return res.status(400).send("no signature");
    }
    if (activity.actor instanceof Array || !activity.actor) {
        return res.status(400).send("actor is more than one AS entity ref");
    }
    const actor = (await resolveApEntity(activity.actor)) as Actor;
    if (
        !httpSignature.verifySignature(
            httpSignature.parseRequest(req),
            actor.publicKey!!.publicKeyPem
        )
    ) {
        console.error(
            `signature check failed, actor: ${actor}, provided: ${signature}`
        );
        return res
            .status(400)
            .send(`signature check failed, expected: ${actor.publicKey}`);
    }
    if (activity.type == ActivityTypes.FOLLOW) {
        // follow request
        console.log(`sending follow Reject to ${actor.inbox}`);
        if (actor.inbox! instanceof URL) {
            return res.status(400).send("inbox is not a standalone doc");
        }
        await deliveryAPActivity(
            actor.inbox as unknown as URL,
            {
                type: ActivityTypes.REJECT,
                id: new URL(
                    `https://xtexx.eu.org/ap/reject_follows/${encodeURI(
                        actor.id!!.toString()
                    )}`
                ),
                actor: new URL("https://xtexx.eu.org/ap/actor.json"),
                object: activity.id,
                target: actor.id,
            } as Reject
        );
    }
    res.status(200).end();
}
