import Head from 'next/head'
import Link from 'next/link'
import styles from '../styles/Home.module.css'

export default function Home() {
  return (
    <div>
      <Head>
        <title>xtex&apos;s Home</title>
        <meta name="description" content="xtex's Home" />
      </Head>

      <main className={styles.main}>
        <div className={styles.header}>
          <h1 className={styles.title}>xtex</h1>
          <p className={styles.description}>初三，学生</p>
          <div className='links'>
            <ul>
              <li><a href="https://blog.xtexx.ml/about" target="_blank" rel="noreferrer">About</a></li>
              <li><a href="https://blog.xtexx.ml/about/contact.html" target="_blank" rel="noreferrer">Contact</a></li>
            </ul>
          </div>
        </div>
      </main>

      <footer className={styles.footer}>
        <h3 id="links">Links<a href="#links"></a></h3>
        <div className='links'>
          <ul>
            <li><a href="https://blog.xtexx.ml/" target="_blank" rel="noreferrer">Blog</a></li>
            <li><a href="https://status.xtexx.ml/" target="_blank" rel="noreferrer">Status</a></li>
          </ul>
        </div>
        <div className='links'>
          <ul>
            <li><a href="https://github.com/xtexChooser" target="_blank" rel="noreferrer">GitHub</a></li>
          </ul>
        </div>
        <p>
          <Link href="/site_about" target="_blank" rel="noreferrer">About this site</Link>
        </p>
      </footer>
    </div>
  )
}
