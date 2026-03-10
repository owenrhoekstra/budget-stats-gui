import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
	{
		path: "/",
		redirect: "/Landing-Page",
	},
	{
		path: "/Landing-Page",
		component: () => import("../views/Landing Pages/Landing-Page.vue"),
	},
	{
		path: "/Settings-Landing",
		component: () => import("../views/Landing Pages/Settings-Landing.vue"),
	},
	{
		path: "/Script-Upload",
		component: () =>
			import("../views/Work Pages/Settings/Settings-Script-Upload-View.vue"),
	},
	{
		path: "/View-Data-Landing",
		component: () => import("../views/Landing Pages/View-Data-Landing.vue"),
	},
	{
		path: "/Change-Data-Landing",
		component: () => import("../views/Landing Pages/Change-Data-Landing.vue"),
	},
	{
		path: "/Data-View-Account-Selection",
		component: () =>
			import("../views/Landing Pages/Data-View-Account-Selection.vue"),
	},
	{
		path: "/develop",
		component: () => import("../views/Develop.vue"),
	},
	{
		path: "/WealthSimple-Data-View",
		component: () =>
			import("../views/Work Pages/WealthSimple/WealthSimple-Data-View.vue"),
	},
	{
		path: "/settings-script-upload-view",
		component: () =>
			import("../views/Work Pages/Settings/Settings-Script-Upload-View.vue"),
	},
	{
		path: "/settings-landing",
		component: () => import("../views/Landing Pages/Settings-Landing.vue"),
	},
	{
		path: "/chequing-data-view",
		component: () => import("../views/Work Pages/Chequing/Tangerine-Chequing-Data-View.vue"),
	},
	{
		path: "/investia-TFSA-data-view",
		component: () => import("../views/Work Pages/Investia TFSA/Investia-TFSA-Data-View.vue"),
	},
];

const router = createRouter({
	history: createWebHashHistory(),
	routes,
});

export default router;
