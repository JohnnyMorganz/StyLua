import fetch from "node-fetch";

const RELEASES_URL =
  "https://api.github.com/repos/JohnnyMorganz/StyLua/releases";
const RELEASES_URL_LATEST = RELEASES_URL + "/latest";

export interface GithubRelease {
  assets: {
    downloadUrl: string;
    name: string;
  }[];
  tagName: string;
  htmlUrl: string;
}

async function fetchJson(url: string): Promise<any> {
  const response = await fetch(url);
  return response.json();
}

function releaseFromJson(json: any): GithubRelease {
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

export const getRelease = async (version: string): Promise<GithubRelease> => {
  if (version === "latest") {
    const json = await fetchJson(RELEASES_URL_LATEST);
    return releaseFromJson(json);
  }

  version = version.startsWith("v") ? version : "v" + version;
  const json = await fetchJson(RELEASES_URL);
  const releases: GithubRelease[] = Array.isArray(json)
    ? json.map(releaseFromJson)
    : [];
  for (const release of releases) {
    if (release.tagName.startsWith(version)) {
      return release;
    }
  }

  throw new Error(`No release version matches ${version}.`);
};
