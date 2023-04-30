function log(text) {
    console.info(`[lumberjack-solver] ${text}`)
}

browser.webRequest.onBeforeRequest.addListener(
    function (req) {
        if (req.url.startsWith('https://tbot.xyz/lumber/js/main.min.js')) {
            log('matched main.min.js request: ' + req.url)

            let filter = browser.webRequest.filterResponseData(req.requestId);
            let decoder = new TextDecoder('utf-8');
            let encoder = new TextEncoder();

            filter.ondata = (event) => {
                let str = decoder.decode(event.data, { stream: true })

                str = str.replace('da=[0,0];', 'da=[0,0];/*LJS injected*/window.ljs_da=da;')

                filter.write(encoder.encode(str))
                log('filtered main.min.js data')
            }

            filter.onstop = (event) => {
                filter.write(encoder.encode("/*LJS injected*/window.ljs_on=true;"))
                filter.disconnect()
            }
        }
        return {};
    },
    { urls: ["<all_urls>"] },
    ["blocking"]
);