<template>
  <!-- <div class="flex"> -->
    <aside class="top-0 left-0 p-0 m-0">
    <div class="top-0 left-0 w-64 m-0 h-full bg-slate-800 text-white">
      <ul>
          <MoleculeSideNavItem 
            v-for="item in items" 
            :key="item.id" 
            :value="item.value"
            :selected="selectedItemId === item.id"
            @click="handleSelect(item)"
          />
      </ul>
    </div>
  </aside>
</template>

<script>
import MoleculeSideNavItem from '../molecules/MoleculeSideNavItem.vue';
export default {
  components: {
    MoleculeSideNavItem
  },
  data() {
    return {
      items: [
        { id: 1, value: 'today' },
        { id: 2, value: 'tomorrow' },
        { id: 3, value: 'inbox' },
        { id: 4, value: 'projects' },
        { id: 5, value: 'tags' },
        { id: 6, value: 'search' }
      ],
      selectedItemId: 1
    }
  },
  methods: {
    handleSelect(item) {
      this.selectedItemId = item.id;
      this.$router.push({ name: item.value });
    }
  }
}
</script>
