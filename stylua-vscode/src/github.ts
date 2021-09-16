import {
  authentication,
  Disposable,
  AuthenticationSession,
  AuthenticationSessionsChangeEvent,
  window,
} from "vscode";
import fetch, { Headers } from "node-fetch";

const RELEASES_URL =
  "https://api.github.com/repos/JohnnyMorganz/StyLua/releases";
const RELEASES_URL_LATEST = RELEASES_URL + "/latest";
const SCOPES: string[] = [];

export interface GitHubRelease {
  assets: {
    downloadUrl: string;
    name: string;
  }[];
  tagName: string;
  htmlUrl: string;
}

async function fetchJson(
  url: string,
  token: string | undefined = undefined
): Promise<any> {
  const headers = new Headers();
  if (token) {
    headers.set("Authorization", `token ${token}`);
  }
  const response = await fetch(url, { headers: headers });
  return response.json();
}

function releaseFromJson(json: any): GitHubRelease {
  if (typeof json !== "object") {
    return {
      assets: [],
      tagName: "",
      htmlUrl: "",
    };
  }
  return {
    assets: Array.isArray(json.assets)
      ? json.assets.map((asset: any) => ({
          downloadUrl:
            typeof asset.browser_download_url === "string"
              ? asset.browser_download_url
              : "",
          name: typeof asset.name === "string" ? asset.name : "",
        }))
      : [],
    tagName: typeof json.tag_name === "string" ? json.tag_name : "",
    htmlUrl: typeof json.html_url === "string" ? json.html_url : "",
  };
}

class Credential {
  private _session: string | undefined;
  private _token: string | undefined;

  constructor() {
    this.set();
  }

  public get authenticated(): boolean {
    return !!this._session;
  }

  public get token(): string | undefined {
    return this._token;
  }

  public get session(): string | undefined {
    return this._session;
  }

  public set(session: AuthenticationSession | undefined = undefined) {
    if (session) {
      this._session = session.id;
      this._token = session.accessToken;
    } else {
      this._session = undefined;
      this._token = undefined;
    }
  }
}

export class GitHub implements Disposable {
  private disposables: Disposable[] = [];
  private credential: Credential = new Credential();

  constructor() {
    this.disposables.push(
      authentication.onDidChangeSessions(
        (event: AuthenticationSessionsChangeEvent) => {
          if (event.provider.id === "github") {
            this.credential.set();
            this.authenticate(false);
          }
        }
      )
    );
    this.authenticate(false);
  }

  dispose() {
    this.disposables.forEach((d) => d.dispose());
  }

  public get authenticated(): boolean {
    return this.credential.authenticated;
  }

  public async authenticate(create: boolean = true): Promise<boolean> {
    try {
      const token = await authentication.getSession("github", SCOPES, {
        createIfNone: create,
      });
      this.credential.set(token);
    } catch (e) {
      if (
        e instanceof Error &&
        e.message === "User did not consent to login."
      ) {
        this.credential.set();
        return false;
      } else {
        window.showErrorMessage(`Failed to authenticate with GitHub: ${e}`);
      }
    }
    return this.credential.authenticated;
  }

  public async getRelease(version: string): Promise<GitHubRelease> {
    if (version === "latest") {
      const json = await fetchJson(RELEASES_URL_LATEST, this.credential.token);
      return releaseFromJson(json);
    }

    version = version.startsWith("v") ? version : "v" + version;
    const json = await fetchJson(RELEASES_URL, this.credential.token);
    const releases: GitHubRelease[] = Array.isArray(json)
      ? json.map(releaseFromJson)
      : [];
    for (const release of releases) {
      if (release.tagName.startsWith(version)) {
        return release;
      }
    }

    throw new Error(`No release version matches ${version}.`);
  }
}
