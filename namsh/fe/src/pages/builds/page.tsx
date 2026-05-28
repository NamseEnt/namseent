import { useState } from "react";
import type { Props } from "./.props";
import { listBuilds } from "../../actions/.generated/list_builds";
import { requestPdbUpload } from "../../actions/.generated/request_pdb_upload";
import { requestPdbDownload } from "../../actions/.generated/request_pdb_download";
import { confirmPdbUploaded } from "../../actions/.generated/confirm_pdb_uploaded";

type BuildRow = {
    buildId: string;
    createdAt: string;
    uploadedBy: number;
    pdbUploaded: boolean;
    pdbSize: number | null;
};

type IssuedKey = {
    buildId: string;
    hmacKeyHex: string;
    presignedPutUrl: string;
};

export default function BuildsPage(props: Props) {
    const [builds, setBuilds] = useState<BuildRow[]>(
        props.builds.map((b) => ({
            buildId: b.buildId,
            createdAt: b.createdAt.toISOString(),
            uploadedBy: b.uploadedBy,
            pdbUploaded: b.pdbUploaded,
            pdbSize: b.pdbSize ?? null,
        })),
    );
    const [newBuildId, setNewBuildId] = useState("");
    const [issued, setIssued] = useState<IssuedKey | null>(null);
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [copiedHmac, setCopiedHmac] = useState(false);
    const [copiedUrl, setCopiedUrl] = useState(false);

    async function refresh() {
        const res = await listBuilds({});
        if (res.t === "Ok") {
            setBuilds(
                res.builds.map((b) => ({
                    buildId: b.buildId,
                    createdAt: b.createdAt.toISOString(),
                    uploadedBy: b.uploadedBy,
                    pdbUploaded: b.pdbUploaded,
                    pdbSize: b.pdbSize ?? null,
                })),
            );
        }
    }

    async function onCreate(e: React.FormEvent) {
        e.preventDefault();
        if (!newBuildId.trim() || busy) return;
        setBusy(true);
        const res = await requestPdbUpload({ build_id: newBuildId.trim() });
        setBusy(false);
        if (res.t === "Ok") {
            setIssued({
                buildId: res.buildId,
                hmacKeyHex: res.hmacKeyHex,
                presignedPutUrl: res.pdbPresignedPutUrl,
            });
            setCopiedHmac(false);
            setCopiedUrl(false);
            setNewBuildId("");
            refresh();
        } else if (res.t === "InvalidBuildId") {
            setError("invalid build id (allowed: [A-Za-z0-9._-], <=128 chars)");
        } else if (res.t === "Error") {
            setError(res.message);
        } else {
            setError("not signed in");
        }
    }

    async function onConfirm(buildId: string) {
        if (busy) return;
        setBusy(true);
        const res = await confirmPdbUploaded({ build_id: buildId });
        setBusy(false);
        if (res.t === "Ok") {
            refresh();
        } else if (res.t === "NotUploaded") {
            setError(`R2 does not have a PDB for ${buildId} yet.`);
        } else if (res.t === "Error") {
            setError(res.message);
        }
    }

    async function onDownload(buildId: string) {
        if (busy) return;
        setBusy(true);
        const res = await requestPdbDownload({ build_id: buildId });
        setBusy(false);
        if (res.t === "Ok") {
            window.location.href = res.presignedGetUrl;
        } else {
            setError("not found");
        }
    }

    async function onRotateUrl(buildId: string) {
        if (busy) return;
        setBusy(true);
        const res = await requestPdbUpload({ build_id: buildId });
        setBusy(false);
        if (res.t === "Ok") {
            setIssued({
                buildId: res.buildId,
                hmacKeyHex: res.hmacKeyHex,
                presignedPutUrl: res.pdbPresignedPutUrl,
            });
            setCopiedHmac(false);
            setCopiedUrl(false);
        }
    }

    return (
        <div style={{ maxWidth: 1080, margin: "2rem auto", fontFamily: "system-ui" }}>
            <Header githubLogin={props.githubLogin} />
            <h1>Builds</h1>

            <h2>Register a new build</h2>
            <form onSubmit={onCreate} style={{ display: "flex", gap: 8 }}>
                <input
                    type="text"
                    placeholder="build id (git hash, semver, ...)"
                    value={newBuildId}
                    onChange={(e) => setNewBuildId(e.target.value)}
                    style={{ flex: 1, padding: 6 }}
                    disabled={busy}
                />
                <button type="submit" disabled={busy || !newBuildId.trim()}>
                    Register
                </button>
            </form>

            {issued && (
                <div
                    style={{
                        marginTop: 12,
                        padding: 12,
                        border: "1px solid #888",
                        background: "#fffbe6",
                    }}
                >
                    <p>
                        <strong>Build {issued.buildId} ready.</strong>
                    </p>
                    <p>
                        HMAC key (bake into your game build; never displayed again):
                    </p>
                    <span style={visuallyHidden}>{issued.hmacKeyHex}</span>
                    <button
                        onClick={async () => {
                            await navigator.clipboard.writeText(issued.hmacKeyHex);
                            setCopiedHmac(true);
                        }}
                    >
                        {copiedHmac ? "Copied HMAC" : "Copy HMAC key"}
                    </button>
                    <p style={{ marginTop: 12 }}>
                        Upload PDB with a single PUT to this URL (expires in 10 minutes):
                    </p>
                    <button
                        onClick={async () => {
                            await navigator.clipboard.writeText(issued.presignedPutUrl);
                            setCopiedUrl(true);
                        }}
                    >
                        {copiedUrl ? "Copied URL" : "Copy PUT URL"}
                    </button>
                    <span style={visuallyHidden}>{issued.presignedPutUrl}</span>
                    <div style={{ marginTop: 8 }}>
                        <button onClick={() => setIssued(null)}>Done</button>
                    </div>
                </div>
            )}

            <h2 style={{ marginTop: 32 }}>Existing builds</h2>
            {builds.length === 0 ? (
                <p>No builds registered yet.</p>
            ) : (
                <table style={{ width: "100%", borderCollapse: "collapse" }}>
                    <thead>
                        <tr>
                            <th style={cell}>Build id</th>
                            <th style={cell}>Created</th>
                            <th style={cell}>By</th>
                            <th style={cell}>PDB</th>
                            <th style={cell}></th>
                        </tr>
                    </thead>
                    <tbody>
                        {builds.map((b) => (
                            <tr key={b.buildId}>
                                <td style={cell}><code>{b.buildId}</code></td>
                                <td style={cell}>{new Date(b.createdAt).toLocaleString()}</td>
                                <td style={cell}>{b.uploadedBy}</td>
                                <td style={cell}>
                                    {b.pdbUploaded
                                        ? `${b.pdbSize ?? "?"} bytes`
                                        : <span style={{ color: "#999" }}>not uploaded</span>}
                                </td>
                                <td style={cell}>
                                    <button onClick={() => onRotateUrl(b.buildId)} disabled={busy}>
                                        Get PUT URL
                                    </button>
                                    {" "}
                                    <button onClick={() => onConfirm(b.buildId)} disabled={busy}>
                                        Refresh PDB status
                                    </button>
                                    {" "}
                                    {b.pdbUploaded && (
                                        <button onClick={() => onDownload(b.buildId)} disabled={busy}>
                                            Download
                                        </button>
                                    )}
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            )}

            {error && (
                <p style={{ color: "crimson", marginTop: 16 }}>{error}</p>
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

const cell: React.CSSProperties = {
    padding: 8,
    borderBottom: "1px solid #eee",
    textAlign: "left",
};

const visuallyHidden: React.CSSProperties = {
    position: "absolute",
    width: 1,
    height: 1,
    margin: -1,
    padding: 0,
    overflow: "hidden",
    clip: "rect(0, 0, 0, 0)",
    whiteSpace: "nowrap",
    border: 0,
};
