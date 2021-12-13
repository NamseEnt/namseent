import koa from "koa";
import serve from "koa-static";
import Router from "koa-router";
import websockify from "koa-websocket";
import fs from "fs-extra";
import path from "path";
import { parse } from "node-html-parser";
import ws from "ws";
import { outdir } from "../buildOption";
import hotReloadScript from "./hotRealoadScript";

export default class DevServer {
  private readonly server = websockify(new koa());
  private readonly router = new Router();
  private readonly websockets: Set<ws> = new Set();

  constructor() {
    this.router.get(['/index.html', '/'], this.serveHotReloadInjectedIndexHtml);

    this.server
      .use(this.router.routes())
      .use(serve(outdir))

    this.server.ws
      .use(async (ctx, next) => {
        this.websockets.add(ctx.websocket);
        ctx.websocket.onclose = () => this.websockets.delete(ctx.websocket);
        await next();
      })
  }

  public start(port: number = 1234) {
    this.server.listen(port);
  }

  public emitHotReload() {
    this.websockets.forEach(websocket => websocket.send('hotReload'));
  }

  private serveHotReloadInjectedIndexHtml: Router.IMiddleware = async (ctx, next) => {
    const html = parse(await fs.readFile(path.join(outdir, 'index.html'), 'utf8'));
    const body = html.querySelector('body');
    if (!body) {
      await next();
    }
    body.insertAdjacentHTML('beforeend', `<script>${hotReloadScript}</script>`);
    await next();

    ctx.body = html.toString();
  }
}
