<template>
  <!-- <li> -->
    <div 
      class="flex h-16 sm:p-4 hover:bg-slate-400 relative truncate" 
      :class="[selected ? 'bg-slate-500' : '', showNavMobile ? 'p-4' : '']"
    >  
      <div 
        class="flex h-8 w-8 mx-auto sm:mx-2 my-auto text-right"
        :class="[showNavMobile ? 'mx-2' : '']"
        v-if="iconList.includes(value)"
      >
        <SunIcon v-if="value==='today'"/>
        <MoonIcon v-else-if="value==='tomorrow'"/>
        <IconCircle v-else-if="value==='later'" class="mr-1"/>
        <BellSnoozeIcon v-else-if="value==='snoozed'"/>
        <InboxIcon v-else-if="value==='inbox'"/>
        <FolderIcon v-else-if="value==='projects'"/>
        <TagIcon v-else-if="value==='tags'"/>
        <MagnifyingGlassIcon v-else-if="value==='search'"/>
        <ArrowTopRightOnSquareIcon v-else-if="value==='logout'"/>
      </div>
      <div 
        v-if="!showNavMobile && !iconList.includes(value)"
        class="sm:hidden text-sm text-left text-balance px-[5px] my-auto truncate"
      >
        {{value}}
      </div>
      <div 
        class="sm:flex items-center my-auto mx-2 w-full bg"
        :class="showNavMobile ? 'flex' : 'hidden'"
      >
        <AtomText>{{value}}</AtomText>
      </div>
      <div v-if="showCount()"
        class="bg-slate-400 items-center rounded-full w-5 h-5 my-auto flex-shrink-0 sm:flex"
        :class="showNavMobile ? 'flex' : 'hidden'"
      >
        <p class="mx-auto my-auto text-xs"> {{ taskCount }} </p>
      </div>
      <div v-if="showCount()"
        class="bg-slate-400 items-center rounded-full w-5 h-5 absolute right-[3px] flex flex-shrink-0 sm:hidden"
        :class="showNavMobile ? 'hidden' : 'flex'"
      >
        <p class="mx-auto my-auto text-xs"> {{ taskCount }} </p>
      </div>
      <div 
        v-if="selected && type==='projects'" 
        class="my-auto items-center sm:block" 
        :class="showNavMobile ? 'block' : 'hidden'"
        @click.stop="editProject"
      >
        <Cog6ToothIcon class="h-6 w-6"/>
      </div>
    </div>
  <!-- </li> -->
</template>

<script>
import { SunIcon, MoonIcon, FolderIcon, InboxIcon, TagIcon, MagnifyingGlassIcon, BellSnoozeIcon, ArrowTopRightOnSquareIcon, Cog6ToothIcon } from '@heroicons/vue/24/outline';
import IconCircle from '../icons/IconCircle.vue';
import AtomText from '@/components/atoms/AtomText.vue';
import { getTodayTasks, getInboxTasks } from '@/db/tasks';
export default {
  props: [ "id", "value", "selected", "showNavMobile", "type" ],
  components: {
    SunIcon,
    MoonIcon,
    IconCircle,
    FolderIcon,
    InboxIcon,
    TagIcon,
    MagnifyingGlassIcon,
    AtomText,
    BellSnoozeIcon,
    ArrowTopRightOnSquareIcon,
    Cog6ToothIcon
  },
  data() {
    return {
      logoutIcon: null,
      iconList: [
        'today', 'tomorrow', 'later', 'snoozed', 'inbox', 'projects', 'tags', 'search', 'logout'
      ],
      taskCount: 0
    }
  },
  methods: {
    editProject() {
      this.$emit("editProject", this.id);
    },
    showCount() {
      // return !this.selected && this.taskCount > 0;
      return this.taskCount > 0;
    },
    async getCount() {
      if (this.value === 'inbox') {
        const tasks = await getInboxTasks(this.$store.getters.userInboxId);
        this.taskCount = tasks?.filter((task) => !task.completed)?.length || 0;
      } else if (this.value === 'today') {
        const tasks = await getTodayTasks();
        this.taskCount = tasks?.filter((task) => !task.completed)?.length || 0;
      }
    }
  },
  mounted() {
    if (this.value === 'logout') {
      this.logoutIcon = this.$store.getters.userIcon;
    }
    this.getCount();
    this.$store.subscribe((mutation,) => {
      const mutations = ["setTasks", "updateTask", "deleteTask", "addTask",]
      if (mutations.includes(mutation.type)) {
        this.getCount();
      }
    });
  },
}
</script>
