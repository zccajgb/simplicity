<template>
  <div class="flex flex-col overflow-y-scroll h-full">
    <div v-show="header" class="mb-2">
      <div class="inline-flex items-center w-full min-h-8 bg-slate-700 mb-0">  
        <div class="h-full p-4 hover:bg-slate-400" @click="$emit('back')">
          <ArrowLeftIcon class="h-6 w-6"/>
        </div>
        <div v-if="showNavMobile" class="inline-flex ml-6 items-center justify-center">  
          <p class="text-lg" >{{header}}</p>
        </div>
      </div>
      <div class="sm:flex" :class="showNavMobile ? '' : 'hidden'">
        <AtomSearchBar @input="$emit('filterItems', $event)" ref="search" />
      </div>
    </div>
    <MoleculeSideNavItem
    v-for="item in items" 
    :key="item._id" 
    :value="item.name"
    :selected="selectedItemId === item._id"
    :id="item._id"
    @click="$emit('select', item)"
    :showNavMobile="showNavMobile"
    :type="type"
    @editProject="(value) => $emit('editProject', value)"
    />
    <div v-if="addable" class="">
      <AtomAddInput 
        ref="addItem" 
        :saveFunction="handleAdd" 
        :defaultText="$refs?.search?.$refs?.input" 
        v-model="showAdd" 
        class="bg-slate-700"
      />
    </div>
    <div class="mb-0 mt-auto">
      <div v-if="addable" class="w-1/3 mb-10 ml-auto mr-6">
        <AtomAddButton v-model="showAdd" :focusRef="$refs.addItem" :lightMode="false"/>
      </div>
    </div>
  </div>
</template>

<script>

import AtomSearchBar from '@/components/atoms/AtomSearchBar.vue';
import MoleculeSideNavItem from '@/components/molecules/MoleculeSideNavItem.vue';
import { ArrowLeftIcon } from '@heroicons/vue/24/outline';
import AtomAddButton from '@/components/atoms/AtomAddButton.vue';
import AtomAddInput from '@/components/atoms/AtomAddInput.vue';

export default {
  components: {
    MoleculeSideNavItem,
    ArrowLeftIcon,
    AtomSearchBar,
    AtomAddButton,
    AtomAddInput
  },
  emits: ['select', 'back', 'filterItems', 'add'],
  props: {
    modelValue: {
      type: Array,
      required: true,
    },
    selectedItemId: {
      type: Number,
      required: false
    },
    header: {
      type: String,
      required: false
    },
    addable: {
      type: Boolean,
      default: false
    },
    showNavMobile: {
      type: Boolean,
      default: false
    },
    type: {
      type: String,
      required: true
    }
  },
  data() {
    return {
      showAdd: false
    }
  },
  methods: {
    handleAdd($event) {
      this.$emit("add", $event.target.value);
    },
  },
  computed: {
    items: {
      get() {
        return this.modelValue;
      },
      set(value) {
        this.$emit('update:modelValue', value);
      },
    },
  }
}
</script>