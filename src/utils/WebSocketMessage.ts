export class WebSocketMessage {
  readonly groupId: number;
  readonly type: string;
  readonly payload: any;

  constructor(json: object) {
    this.groupId = json["groupId"];
    this.type = json["type"];
    this.payload = json["payload"];
  }
}
