import { Router } from "@layer0/core";
import { nextRoutes } from "@layer0/next";

module.exports = new Router()
  .match("/service-worker.js", ({ serviceWorker }) => {
    return serviceWorker(".next/static/service-worker.js");
  })
  .match("/protected/static", async ({ verifyJWT }) => {
    await verifyJWT({
      cookie: process.env.JWT_COOKIE || "next-auth.session-token",
      secret: process.env.SIGNING_KEY,
      redirectTo: "/api/auth/signin",
    });
  })
  .match("/_next/data/:version/protected/:page*", async ({ verifyJWT }) => {
    await verifyJWT({
      cookie: process.env.JWT_COOKIE || "next-auth.session-token",
      secret: process.env.SIGNING_KEY,
      redirectTo: "/api/auth/signin",
    });
  })
  .use(nextRoutes); // automatically adds routes for all files under /pages
