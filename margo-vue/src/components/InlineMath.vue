<template>
    <span v-html="math"></span>
</template>

<style lang="sass" scoped>

</style>
<script setup lang="ts">
import katex from 'katex';
const props = defineProps<{ content: string }>()

var math = "";
try {
    math = katex.renderToString(props.content)
} catch (error) {
    console.error(error);
};
</script>