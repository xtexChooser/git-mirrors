import type { NextApiRequest, NextApiResponse } from "next";
import TelegramBot from "node-telegram-bot-api";
import { NtfyClient } from "ntfy";
import { env } from "process";
import { sendToNtfy } from "../../util/ntfy";

export default async function handler(
    req: NextApiRequest,
    res: NextApiResponse<string>
) {
    const body = req.body;
    if (req.headers['x-misskey-hook-secret'] != env['XTEX_HOME_MISSKEY_SECRET']) {
        throw 'invalid secret'
    }

    const bot = new TelegramBot(env['XTEX_HOME_MISSKEY_TG_TOKEN'] as string, {});
    switch (body['type'] as string) {
        case 'note': {
            const note = body['body']['note'] as any;
            if (body['userId'] == '8pjo5pvnqn' && note['visibility'] == 'public') {
                await sendToNtfy({
                    topic: 'xtex-logs',
                    message: `Forwarding Misskey note to TG: ${note['id']}`
                });
                const text = note['renote']?.['text'] || note['text'];
                await bot.sendMessage('-1001657723727', `${text}\n(src: https://neko.ci/notes/${note['id']})`);
            }
        }
    }

    res.status(200).send('xtex-home: done');
}
