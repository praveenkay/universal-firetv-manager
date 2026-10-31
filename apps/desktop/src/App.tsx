import { useMemo, useState } from "react";

export type Launcher = {
  id: string;
  name: string;
  source_url: string;
  license: string;
  open_source: boolean;
  user_supplied: boolean;
  status: "recommended" | "supported" | "experimental";
  warning?: string | null;
};

export const launchers: Launcher[] = [
  { id: "projectivy", name: "Projectivy Launcher", source_url: "https://github.com/spocky/miproja1/releases", license: "User-supplied; premium license supported", open_source: false, user_supplied: true, status: "supported", warning: "Select your own official APK; it is never bundled." },
  { id: "dumbtv-home", name: "dumbtv Home", source_url: "https://github.com/humblemuzzu/dumbtv", license: "GPL-3.0", open_source: true, user_supplied: false, status: "recommended" },
  { id: "ltv-launcher", name: "LTvLauncher", source_url: "https://github.com/leanbitlab-org/LtvLauncher", license: "See upstream", open_source: true, user_supplied: false, status: "experimental", warning: "Test Home and Settings before keeping changes." },
  { id: "firetv-web-launcher", name: "Fire TV Web Launcher", source_url: "https://github.com/pickdani/fire-tv-web-launcher", license: "See upstream", open_source: true, user_supplied: false, status: "experimental", warning: "Only available for validated Fire OS 8 profiles." }
];

const steps = ["Enable ADB", "Connect", "Compatibility", "Launcher", "Backup", "Review", "Results"];

export function App() {
  const [step, setStep] = useState(0);
  const [address, setAddress] = useState("");
  const [selected, setSelected] = useState("projectivy");
  const numericAddress = useMemo(() => /^(?:\d{1,3}\.){3}\d{1,3}(?::\d+)?$/.test(address), [address]);
  const canContinue = step !== 1 || numericAddress;

  return <main>
    <header><p className="eyebrow">Universal Fire TV Manager</p><h1>Keep the TV experience. Remove the clutter.</h1><p>Backup-first launcher setup, conservative debloat, health checks, uninstall, and restore.</p></header>
    <nav aria-label="Setup progress">{steps.map((name, index) => <span className={index === step ? "active" : index < step ? "done" : ""} key={name}>{index + 1}. {name}</span>)}</nav>
    <section className="panel">
      {step === 0 && <><h2>Enable ADB debugging</h2><ol><li>Open Fire TV Settings → My Fire TV → Developer Options.</li><li>Enable ADB Debugging.</li><li>Keep the Fire TV and this computer on the same trusted network.</li></ol><p className="notice">You will approve this computer on the television. The app never scans your network.</p></>}
      {step === 1 && <><h2>Connect by IP address</h2><label>Fire TV IP address<input value={address} onChange={event => setAddress(event.target.value)} placeholder="192.168.1.50" /></label>{address && !numericAddress && <p role="alert">Enter a numeric IPv4 address, optionally followed by a port.</p>}<p>Waiting for authorization? Accept the RSA prompt on the TV, then retry.</p></>}
      {step === 2 && <><h2>Compatibility result</h2><div className="result"><strong>Base-safe mode</strong><p>Unknown builds retain Amazon Settings, accounts, DRM, Appstore, keyboard, Bluetooth, HDMI inputs, audio, and OTA services.</p></div></>}
      {step === 3 && <><h2>Choose a launcher</h2><p>Projectivy is supported with your own APK. Free and open-source choices remain available.</p><div className="cards">{launchers.map(launcher => <label className={selected === launcher.id ? "card selected" : "card"} key={launcher.id}><input type="radio" name="launcher" value={launcher.id} checked={selected === launcher.id} onChange={() => setSelected(launcher.id)} /><span><strong>{launcher.name}</strong><small>{launcher.status} · {launcher.license}</small>{launcher.warning && <em>{launcher.warning}</em>}<a href={launcher.source_url}>Upstream source</a></span></label>)}</div></>}
      {step === 4 && <><h2>Create the mandatory baseline backup</h2><p>The backup records device identity, package state, launcher/settings values, and hashes. Retail ADB cannot copy private app logins, DRM keys, or a flashable Amazon OS image.</p><button>Create baseline backup</button></>}
      {step === 5 && <><h2>Review changes</h2><p>Selected launcher: <strong>{launchers.find(item => item.id === selected)?.name}</strong></p><p>No protected service will be disabled. A matching plan hash and explicit confirmation are required.</p><details><summary>Advanced plan details</summary><pre>Install verified launcher APK{"\n"}Preserve Settings, Appstore, accounts, Bluetooth, HDMI, audio, keyboard, DRM, OTA{"\n"}Run health checks and roll back on failure</pre></details></>}
      {step === 6 && <><h2>Verify the result</h2><p>Run remote, Home, Settings, Appstore, accounts, keyboard, Bluetooth, HDMI input, storage, and audio checks.</p><div className="actions"><button>Run health checks</button><button className="secondary">Uninstall changes</button><button className="secondary">Restore backup</button></div></>}
      <footer><button className="secondary" disabled={step === 0} onClick={() => setStep(value => value - 1)}>Back</button><button disabled={step === steps.length - 1 || !canContinue} onClick={() => setStep(value => value + 1)}>Continue</button></footer>
    </section>
  </main>;
}
