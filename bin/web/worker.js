export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    
    // Try to serve the requested asset
    let response = await env.ASSETS.fetch(request);
    
    // WASM 文件强制设置正确的 Content-Type
    if (url.pathname.endsWith('.wasm')) {
      response = new Response(response.body, response);
      response.headers.set('Content-Type', 'application/wasm');
      return response;
    }

    // If the asset is not found (404), fallback to index.html for SPA routing
    if (response.status === 404) {
      return env.ASSETS.fetch(new URL('/index.html', request.url));
    }

    return response;
  }
};
