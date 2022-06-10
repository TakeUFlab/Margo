<style lang="sass" scoped>
  
</style>
<script setup lang="ts">
defineProps<{ level: string }>()
</script>
<script lang="ts">
import { defineComponent, h } from 'vue';

export default defineComponent({
    render() {
        return h(
            `h${this.$props.level}`,
            null,
            this.$slots
        )
    }
})
</script>