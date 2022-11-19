import Head from 'next/head'
import styles from '../styles/Home.module.css'

export default function SiteAbout() {
    return (
        <div>
            <Head>
                <title>xtex&apos;s Home :: Site About</title>
                <meta name="description" content="xtex's Home :: Site About" />
            </Head>

            <main>
                <h1>xtex&apos;s Home :: Site About</h1>
                <p>
                    This site hosts as the personal homepage for xtex.<br />
                    Source code is available at <a href='git://github.com/xtexChooser/home.git'>git://github.com/xtexChooser/home.git</a> (<a href='#license'>more about the license</a>).
                </p>

                <h2 id="services">Services<a href="#services"></a></h2>
                <h2 id="web-finger">Web Finger<a href="#web-finger"></a></h2>

                <h2 id="license">License<a href="#license"></a></h2>
                <p>
                    This site is open-sourced under GNU Affero General Public License.<br />
                    A copy is attached in the git repository with the filename &quot;LICENSE&quot;.<br />
                    <code>
                        xtex&apos;s Home<br />
                        Copyright (C) 2022 - {new Date().getUTCFullYear()}  xtex<br />
                        <br />
                        This program is free software: you can redistribute it and/or modify<br />
                        it under the terms of the GNU Affero General Public License as published<br />
                        by the Free Software Foundation, either version 3 of the License, or<br />
                        (at your option) any later version.<br />
                        <br />
                        This program is distributed in the hope that it will be useful,<br />
                        but WITHOUT ANY WARRANTY; without even the implied warranty of<br />
                        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the<br />
                        GNU Affero General Public License for more details.<br />
                        <br />
                        You should have received a copy of the GNU Affero General Public License<br />
                        along with this program.  If not, see &lt;https://www.gnu.org/licenses/&gt;.<br />
                    </code>
                </p>
            </main>
        </div>
    )
}
