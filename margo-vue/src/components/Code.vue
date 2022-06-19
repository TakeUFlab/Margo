<template>
    <pre><code v-html="code" class="hljs"></code></pre>
</template>

<style lang="sass" scoped>

</style>
<script setup lang="ts">
import hljs from 'highlight.js';

const prop = defineProps<{ lang?: string, content: string }>();
var code = ""
try {
    if (prop.lang == null) {
        code = hljs.highlightAuto(prop.content).value
    } else {
        code = hljs.highlight(prop.content, { language: prop.lang }).value
    }
} catch (error) {
    console.error(error);
}

</script>