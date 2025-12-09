// Based off https://github.com/cloudflare/binary-install
// Licensed under MIT
const os = require("os");
const axios = require("axios");
const unzip = require("unzipper");
const rimraf = require("rimraf");
const { ProxyAgent } = require('proxy-agent');
const { join } = require("path");
const { existsSync, mkdirSync, createWriteStream } = require("fs");
const { spawnSync } = require("child_process");

const { version: VERSION, repository: REPOSITORY } = require("./package.json");

const SUPPORTED_PLATFORMS = [
  {
    platform: "win32",
    arch: "x64",
    name: "stylua-windows-x86_64",
  },
  {
    platform: "darwin",
    arch: "x64",
    name: "stylua-macos-x86_64",
  },
  {
    platform: "darwin",
    arch: "arm64",
    name: "stylua-macos-aarch64",
  },
  {
    platform: "linux",
    arch: "x64",
    name: "stylua-linux-x86_64",
  },
  {
    platform: "linux",
    arch: "arm64",
    name: "stylua-linux-aarch64",
  },
  {
    platform: "linux",
    arch: "loongarch64",
    name: "stylua-linux-loongarch64",
  },
];

const error = (msg) => {
  console.error(msg);
  process.exit(1);
};

const downloadArtifact = (url, location) => {
  const agent = new ProxyAgent();
  return new Promise((resolve, reject) => {
    axios
      // proxy: false is needed, otherwise axios is trying to overwrite the agent
      // See https://github.com/axios/axios/issues/4531
      .create({ proxy: false, httpAgent: agent, httpsAgent: agent })
      .get(url, { responseType: "stream" })
      .then((res) => res.data.pipe(unzip.Parse()))
      .then((stream) => {
        stream.on("entry", (entry) => {
          //   if (entry.path !== outputFilename) {
          //     entry.autodrain();
          //     return;
          //   }

          entry.pipe(location).on("finish", resolve).on("error", reject);
        });
      })
      .catch(reject);
  });
};

const getDownloadUrl = () => {
  const platform = os.platform();
  const arch = os.arch();
  const supportedInfo = SUPPORTED_PLATFORMS.find(
    (info) => info.platform === platform && info.arch === arch
  );

  if (!supportedInfo) {
    error(`Your platform [${platform} ${arch}] is currently unsupported.`);
  }

  return `${REPOSITORY.url}/releases/download/v${VERSION}/${supportedInfo.name}.zip`;
};

const getInstallDirectory = () => {
  const path = join(__dirname, "bin");
  if (!existsSync(path)) {
    mkdirSync(path, { recursive: true });
  }
  return path;
};

const getBinaryPath = () => {
  const dir = getInstallDirectory();

  if (os.platform() === "win32") {
    return join(dir, "stylua.exe");
  } else {
    return join(dir, "stylua");
  }
};

const install = () => {
  const url = getDownloadUrl();
  console.log(`Downloading release from ${url}`);

  if (existsSync(getBinaryPath())) {
    rimraf.sync(getBinaryPath());
  }

  const location = createWriteStream(getBinaryPath(), {
    mode: 0o755,
  });

  return downloadArtifact(url, location)
    .then(() => console.log("StyLua has been installed!"))
    .catch((e) => {
      error(`Error fetching release: ${e.message}`);
    });
};

const uninstall = () => {
  if (existsSync(getInstallDirectory())) {
    rimraf.sync(getInstallDirectory());
  }
  console.log("StyLua has been uninstalled");
};

const run = () => {
  const binaryPath = getBinaryPath();

  const [, , ...args] = process.argv;
  const options = { cwd: process.cwd(), stdio: "inherit" };

  const result = spawnSync(binaryPath, args, options);

  if (result.error) {
    error(result.error);
  }

  process.exit(result.status);
};

module.exports = {
  install,
  uninstall,
  run,
};
