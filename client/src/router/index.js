import { createRouter, createWebHistory } from 'vue-router'
import TodayView from '../views/TodayView.vue';
import TestView from '../views/TestView.vue';
import TomorrowView from '@/views/TomorrowView.vue';
import InboxView from '@/views/InboxView.vue';
import SearchView from '@/views/SearchView.vue';
import ProjectView from '@/views/ProjectView.vue';
import TagView from '@/views/TagView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: TodayView
    },
    {
      path: '/today',
      name: 'today',
      component: TodayView
    },
    {
      path: '/tomorrow',
      name: 'tomorrow',
      component: TomorrowView
    },
    {
      path: '/inbox',
      name: 'inbox',
      component: InboxView
    },
    {
      path: '/search',
      name: 'search',
      component: SearchView
    },
    {
      path: '/test',
      name: 'test',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: TestView
    },
    {
      path: "/projects/:projectId",
      name: "projects",
      component: ProjectView,
    },
    {
      path: "/tags/:tagId",
      name: "tags",
      component: TagView,
    }
  ]
})

export default router
