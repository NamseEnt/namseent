import { serve } from "bun";
import { watch } from "fs";

const PORT = 3000;

// TypeScript 파일을 JavaScript로 빌드
async function buildGame() {
    console.log("🔨 빌드 중...");
    const proc = Bun.spawn(["bun", "build", "src/index.ts", "--outfile", "public/game.js", "--target", "browser"]);
    await proc.exited;
    console.log("✅ 빌드 완료!");
}

// 초기 빌드
await buildGame();

// 파일 변경 감지
watch("src", { recursive: true }, async (event, filename) => {
    if (filename?.endsWith('.ts')) {
        await buildGame();
    }
});

// 개발 서버 시작
const server = serve({
    port: PORT,
    async fetch(req) {
        const url = new URL(req.url);
        const path = url.pathname === "/" ? "/index.html" : url.pathname;
        
        const file = Bun.file(`./public${path}`);
        
        if (await file.exists()) {
            return new Response(file);
        }
        
        return new Response("Not Found", { status: 404 });
    },
});

console.log(`🚀 개발 서버가 http://localhost:${PORT} 에서 실행중입니다.`);
console.log("📁 src 폴더의 변경사항을 감지하고 자동으로 빌드합니다.");