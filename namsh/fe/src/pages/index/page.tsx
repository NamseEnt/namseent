import type { Props } from "./.props";

export default function IndexPage(props: Props) {
    return (
        <div style={{ maxWidth: 1080, margin: "2rem auto", fontFamily: "system-ui" }}>
            <Header githubLogin={props.githubLogin} />
            <h1>Crashes</h1>
            {props.groups.length === 0 ? (
                <p style={{ color: "#666" }}>No crashes received yet.</p>
            ) : (
                <table style={{ width: "100%", borderCollapse: "collapse" }}>
                    <thead>
                        <tr>
                            <th style={cell}>Stack hash</th>
                            <th style={cell}>Count</th>
                            <th style={cell}>Stored dumps</th>
                            <th style={cell}>Latest build</th>
                            <th style={cell}>Latest version</th>
                            <th style={cell}>Last seen</th>
                            <th style={cell}>First seen</th>
                        </tr>
                    </thead>
                    <tbody>
                        {props.groups.map((g) => (
                            <tr key={g.stackHash}>
                                <td style={cell}>
                                    <a href={`/issues/${encodeURIComponent(g.stackHash)}`}>
                                        <code>{shortHash(g.stackHash)}</code>
                                    </a>
                                </td>
                                <td style={cell}>{g.count}</td>
                                <td style={cell}>{g.storedDumps} / 3</td>
                                <td style={cell}><code>{g.latestBuildId}</code></td>
                                <td style={cell}>{g.latestAppVersion}</td>
                                <td style={cell}>{new Date(g.lastSeen).toLocaleString()}</td>
                                <td style={cell}>{new Date(g.firstSeen).toLocaleString()}</td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            )}
        </div>
    );
}

function Header({ githubLogin }: { githubLogin: string }) {
    return (
        <nav style={{ display: "flex", gap: 16, marginBottom: 24, alignItems: "baseline" }}>
            <a href="/"><strong>namsh</strong></a>
            <a href="/">Crashes</a>
            <a href="/builds">Builds</a>
            <a href="/tokens">Tokens</a>
            <span style={{ marginLeft: "auto", color: "#666" }}>{githubLogin}</span>
        </nav>
    );
}

function shortHash(hash: string): string {
    return hash.length > 12 ? `${hash.slice(0, 12)}…` : hash;
}

const cell: React.CSSProperties = {
    padding: 8,
    borderBottom: "1px solid #eee",
    textAlign: "left",
};
