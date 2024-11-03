import { createRouter, createWebHistory } from 'vue-router'
import TodayView from '../views/TodayView.vue';
import TestView from '../views/TestView.vue';
import TomorrowView from '@/views/TomorrowView.vue';
import InboxView from '@/views/InboxView.vue';
import SearchView from '@/views/SearchView.vue';
import ProjectView from '@/views/ProjectView.vue';
import TagView from '@/views/TagView.vue';
import LoginView from '@/views/LoginView.vue';
import store from '@/store';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/login',
      name: 'login',
      component: LoginView,
      meta: { requiresAuth: false }
    },
    {
      path: '/',
      name: 'home',
      component: TodayView,
      meta: { requiresAuth: true }
    },
    {
      path: '/today',
      name: 'today',
      component: TodayView,
      meta: { requiresAuth: true }
    },
    {
      path: '/tomorrow',
      name: 'tomorrow',
      component: TomorrowView,
      meta: { requiresAuth: true }
    },
    {
      path: '/inbox',
      name: 'inbox',
      component: InboxView,
      meta: { requiresAuth: true }
    },
    {
      path: '/search',
      name: 'search',
      component: SearchView,
      meta: { requiresAuth: true }
    },
    {
      path: '/test',
      name: 'test',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: TestView,
      meta: { requiresAuth: true }
    },
    {
      path: "/projects/:projectId",
      name: "projects",
      component: ProjectView,
      meta: { requiresAuth: true }
    },
    {
      path: "/tags/:tagId",
      name: "tags",
      component: TagView,
      meta: { requiresAuth: true }
    },
  ]
});

router.beforeEach((to, from, next) => {
  const requiresAuth = to.matched.some(record => record.meta.requiresAuth)

  if (requiresAuth && !store.getters.isLoggedIn) {
    next('/login')
  } else {
    next()
  }
});

export default router
