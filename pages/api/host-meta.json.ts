import { Link } from "activitypub-core-types/lib/activitypub";
import type { NextApiRequest, NextApiResponse } from "next";
import site_lrs from "../../data/site_lrs.json";

type Data = {
  links: Link[];
};

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  res.status(200).json({
    links: site_lrs as Link[],
  });
}
