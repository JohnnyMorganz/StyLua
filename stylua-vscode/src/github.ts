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
  return {
    assets: json.assets.map((asset: any) => ({
      downloadUrl: asset.browser_download_url,
      name: asset.name,
    })),
    tagName: json.tag_name,
    htmlUrl: json.html_url,
  };
}

export const getRelease = async (version: string): Promise<GithubRelease> => {
  if (version === "latest") {
    const json = await fetchJson(RELEASES_URL_LATEST);
    return releaseFromJson(json);
  }

  version = version.startsWith("v") ? version : "v" + version;
  const json = await fetchJson(RELEASES_URL);
  const releases: GithubRelease[] = json.map((release: any) =>
    releaseFromJson(release)
  );
  for (const release of releases) {
    if (release.tagName.startsWith(version)) {
      return release;
    }
  }

  throw new Error(`No release version matches ${version}.`);
};
