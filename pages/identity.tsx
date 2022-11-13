import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'

export default function Identity() {
    return (
        <div>
            <Head>
                <title>xtex::Identity</title>
                <meta name="description" content="xtex's Identification Information" />
            </Head>

            <main className={styles.main}>
                <div className={styles.header}>
                    <h1 className={styles.title}>xtex</h1>
                    <p className={styles.description}>初三，学生</p>
                </div>
                <div className={styles.links}>
                    <ul>
                        <li><a href="https://blog.xtexx.ml/about" target="_blank">About</a></li>
                        <li><a href="https://blog.xtexx.ml/about/contact.html" target="_blank">Contact</a></li>
                    </ul>
                </div>
            </main>

            <footer className={styles.footer}>
                <div className={styles.links}>
                    <ul>
                        <li><a href="https://blog.xtexx.ml/" target="_blank">Blog</a></li>
                        <li><a href="https://status.xtexx.ml/" target="_blank">Status</a></li>
                    </ul>
                </div>
                <div className={styles.links}>
                    <ul>
                        <li><a href="https://github.com/xtexChooser" target="_blank">GitHub</a></li>
                    </ul>
                </div>
            </footer>
        </div>
    )
}
