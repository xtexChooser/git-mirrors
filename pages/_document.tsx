import { Html, Head, Main, NextScript } from 'next/document'
import site_lrs from '../data/site_lrs.json'

export default function Document() {
  return (
    <Html>
      <Head>
        {
          site_lrs.filter(lr => lr.href != null)
            .map((lr, index) => <link rel={lr.rel} href={lr.href} key={index} />)
        }
      </Head>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  )
}
