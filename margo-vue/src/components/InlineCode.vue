<template>
    <code v-html="code" class="hljs"></code>
</template>

<style lang="sass" scoped>

</style>
<script setup lang="ts">
import hljs from 'highlight.js';

const prop = defineProps<{ content: string }>();
var code = ""
try {
    code = hljs.highlightAuto(prop.content).value
} catch (error) {
    console.error(error);
}
</script>