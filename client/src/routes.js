import {lazy} from "solid-js"
const routes = [
    {
        path: "/",
        component: lazy(() => import("./pages/HomePage")),
    },
    {
        path: "explore",
        component: lazy(() => import("./pages/ExploreRecipe")),
    },
    {
        path: "saved",
        component: lazy(() => import("./pages/Saved")),
    },
    {
        path: "friends",
        component: lazy(() => import("./pages/Friends")),
    },
    {
        path: "newRecipe",
        component: lazy(() => import("./pages/NewRecipe")),
    },
    {
        path: "myProfile",
        component: lazy(() => import("./pages/MyProfile")),
    },
    {
        path: "ingredients",
        component: lazy(() => import("./pages/IngredientsPage")),
    },
    {
        path: "*404",
        component: lazy(() => import("./pages/NotFound")),
    }
]
export default routes;