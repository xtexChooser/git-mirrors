import { Config, NtfyClient } from "ntfy";
import { env } from "process";

const ntfy = new NtfyClient('https://ntfy.xvnet.eu.org/');

export default ntfy;

export async function sendToNtfy(config: Config) {
    config.authorization = {
        username: '',
        password: env['XTEX_HOME_NTFY_TOKEN']!
    };
    await ntfy.publish(config)
}
