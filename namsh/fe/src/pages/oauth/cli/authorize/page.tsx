import { useState } from "react";
import type { Props } from "./.props";
import { approveCliAuthorization } from "../../../../actions/.generated/approve_cli_authorization";

export default function AuthorizeCliPage(props: Props) {
    const [label, setLabel] = useState(props.defaultLabel);
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState<string | null>(null);

    async function onApprove(e: React.FormEvent) {
        e.preventDefault();
        if (busy) return;
        const trimmed = label.trim();
        if (!trimmed) {
            setError("label cannot be empty");
            return;
        }
        setBusy(true);
        setError(null);
        const res = await approveCliAuthorization({
            redirectUri: props.redirectUri,
            codeChallenge: props.codeChallenge,
            codeChallengeMethod: props.codeChallengeMethod,
            state: props.state,
            label: trimmed,
        });
        if (res.t === "Ok") {
            window.location.replace(res.redirectTo);
            return;
        }
        setBusy(false);
        if (res.t === "InvalidRequest" || res.t === "Error") {
            setError(res.message);
        } else if (res.t === "NotLoggedIn") {
            setError("Not signed in.");
        }
    }

    function onCancel() {
        window.location.replace("/");
    }

    return (
        <div style={{ maxWidth: 540, margin: "3rem auto", fontFamily: "system-ui" }}>
            <h1>Authorize CLI</h1>
            <p>
                A CLI on this machine is requesting access to your namsh account.
            </p>
            <p>
                Signed in as <strong>{props.githubLogin}</strong>.
            </p>

            <div
                style={{
                    margin: "1rem 0",
                    padding: 12,
                    border: "1px solid #ddd",
                    background: "#fafafa",
                    fontSize: 14,
                }}
            >
                <div style={{ marginBottom: 6 }}>
                    <strong>Will redirect back to:</strong>
                </div>
                <code style={{ wordBreak: "break-all" }}>{props.redirectUri}</code>
            </div>

            <form onSubmit={onApprove}>
                <label style={{ display: "block", marginBottom: 4 }}>
                    Label for this CLI token
                </label>
                <input
                    type="text"
                    value={label}
                    onChange={(e) => setLabel(e.target.value)}
                    placeholder="laptop"
                    style={{ width: "100%", padding: 8, marginBottom: 12 }}
                    disabled={busy}
                />
                <div style={{ display: "flex", gap: 8 }}>
                    <button type="submit" disabled={busy || !label.trim()}>
                        {busy ? "Authorizing…" : "Approve"}
                    </button>
                    <button type="button" onClick={onCancel} disabled={busy}>
                        Cancel
                    </button>
                </div>
            </form>

            {error && (
                <p style={{ color: "crimson", marginTop: 16 }}>{error}</p>
            )}
        </div>
    );
}
