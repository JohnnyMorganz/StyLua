import { window, StatusBarAlignment, StatusBarItem, ThemeColor } from "vscode";

export class Status {
  private status: StatusBarItem;

  constructor() {
    this.status = window.createStatusBarItem(
      "stylua.status",
      StatusBarAlignment.Right,
      -1
    );

    this.status.name = "StyLua";
    this.status.text = "StyLua";

    this.status.show();
  }

  public update(err: unknown) {
    if (err) {
      this.status.text = "$(warning) StyLua";
      this.status.tooltip = `Could not format file: ${err}`;
      this.status.backgroundColor = new ThemeColor(
        "statusBarItem.errorBackground"
      );
    } else {
      this.status.text = "$(check) StyLua";
      this.status.tooltip = "File formatted successfully";
      this.status.backgroundColor = new ThemeColor("statusBarItem");
    }
  }
}
