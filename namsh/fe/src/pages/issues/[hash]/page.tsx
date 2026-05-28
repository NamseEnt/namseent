import { useState } from "react";
import type { Props } from "./.props";
import { requestDumpDownload } from "../../../actions/.generated/request_dump_download";

export default function IssuePage(props: Props) {
    if (props.t === "NotFound") {
        return (
            <div style={{ maxWidth: 720, margin: "2rem auto", fontFamily: "system-ui" }}>
                <Header githubLogin={props.githubLogin} />
                <h1>Issue not found</h1>
                <p><code>{props.stackHash}</code></p>
                <p><a href="/">Back to crashes</a></p>
            </div>
        );
    }

    return <IssueDetail data={props} />;
}

type OkProps = Extract<Props, { t: "Ok" }>;

function IssueDetail({ data }: { data: OkProps }) {
    const [error, setError] = useState<string | null>(null);
    const [busy, setBusy] = useState(false);

    async function onDownload(dumpId: string) {
        if (busy) return;
        setBusy(true);
        const res = await requestDumpDownload({ dump_id: dumpId });
        setBusy(false);
        if (res.t === "Ok") {
            window.location.href = res.presignedGetUrl;
        } else if (res.t === "NotFound") {
            setError("dump not found (may not have been uploaded)");
        } else {
            setError("not signed in");
        }
    }

    const ctx = data.latestContext;
    return (
        <div style={{ maxWidth: 1080, margin: "2rem auto", fontFamily: "system-ui" }}>
            <Header githubLogin={data.githubLogin} />
            <h1>Crash <code style={{ fontSize: "0.7em" }}>{data.stackHash}</code></h1>
            <p>
                <strong>{data.count}</strong> hits ·
                first seen {new Date(data.firstSeen).toLocaleString()} ·
                last seen {new Date(data.lastSeen).toLocaleString()}
            </p>

            <h2>Stored dumps ({data.dumps.length} / 3)</h2>
            {data.dumps.length === 0 ? (
                <p>No dumps stored.</p>
            ) : (
                <table style={{ width: "100%", borderCollapse: "collapse" }}>
                    <thead>
                        <tr>
                            <th style={cell}>Dump id</th>
                            <th style={cell}>Build</th>
                            <th style={cell}>Uploaded at</th>
                            <th style={cell}>Client IP</th>
                            <th style={cell}></th>
                        </tr>
                    </thead>
                    <tbody>
                        {data.dumps.map((d) => (
                            <tr key={d.dumpId}>
                                <td style={cell}><code>{d.dumpId}</code></td>
                                <td style={cell}><code>{d.buildId}</code></td>
                                <td style={cell}>{new Date(d.uploadedAt).toLocaleString()}</td>
                                <td style={cell}>{d.clientIp}</td>
                                <td style={cell}>
                                    <button onClick={() => onDownload(d.dumpId)} disabled={busy}>
                                        Download .dmp
                                    </button>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            )}

            <h2 style={{ marginTop: 24 }}>Latest context</h2>
            <table>
                <tbody>
                    <Field label="build_id" value={ctx.buildId} />
                    <Field label="install_id" value={ctx.installId} />
                    <Field label="session_uptime_sec" value={String(ctx.sessionUptimeSec)} />
                </tbody>
            </table>

            {ctx.logTail && (
                <>
                    <h2 style={{ marginTop: 24 }}>log_tail</h2>
                    <pre style={{
                        background: "#f6f6f6",
                        padding: 12,
                        maxHeight: 480,
                        overflow: "auto",
                        fontSize: 12,
                    }}>{ctx.logTail}</pre>
                </>
            )}

            {error && (
                <p style={{ color: "crimson", marginTop: 16 }}>{error}</p>
            )}
        </div>
    );
}

function Field({ label, value }: { label: string; value: string }) {
    return (
        <tr>
            <td style={{ ...cell, fontWeight: 600, paddingRight: 16 }}>{label}</td>
            <td style={cell}>{value}</td>
        </tr>
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

const cell: React.CSSProperties = {
    padding: 8,
    borderBottom: "1px solid #eee",
    textAlign: "left",
};
