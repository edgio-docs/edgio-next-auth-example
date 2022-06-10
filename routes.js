import { Router } from "@layer0/core";
import { nextRoutes } from "@layer0/next";

export default new Router()
  .match("/service-worker.js", ({ serviceWorker }) => {
    return serviceWorker(".next/static/service-worker.js");
  })
  .match("/protected/static", ({ verifyJwt }) => {
    verifyJwt({
      algo: "HS512",
      header: "Authorization",
      cookie: process.env.JWT_COOKIE || "next-auth.session-token",
      secret: process.env.JWT_SECRET,
      redirectInvalid: "/api/auth/signin",
    });
  })
  .match("/_next/data/:version/protected/:page*", ({ verifyJwt }) => {
    verifyJwt({
      algo: "HS512",
      header: "Authorization",
      cookie: process.env.JWT_COOKIE || "next-auth.session-token",
      secret: process.env.JWT_SECRET,
      redirectInvalid: "/api/auth/signin",
    });
  })
  .use(nextRoutes); // automatically adds routes for all files under /pages
