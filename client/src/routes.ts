import { lazy } from "solid-js";
import type { RouteDefinition } from "@solidjs/router";

const routes: RouteDefinition[] = [
    {
        path: "/",
        component: lazy(() => import("./pages/Home")),
    },
    {
        path: "/explore",
        component: lazy(() => import("./pages/Explore")),
    },
    {
        path: "/saved",
        component: lazy(() => import("./pages/Saved")),
    },
    {
        path: "/friends",
        component: lazy(() => import("./pages/Friends")),
    },
    {
        path: "/newRecipe",
        component: lazy(() => import("./pages/NewRecipe")),
    },
    {
        path: "/myProfile",
        component: lazy(() => import("./pages/MyProfile")),
    },
    {
        path: "/ingredients",
        component: lazy(() => import("./pages/Ingredients")),
    },
    {
        path: "/units",
        component: lazy(() => import("./pages/Units")),
    },
    {
        path: "*404",
        component: lazy(() => import("./pages/NotFound")),
    }
];

export default routes;
