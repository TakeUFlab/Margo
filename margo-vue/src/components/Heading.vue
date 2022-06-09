<style lang="sass" scoped>
</style>
<script setup lang="ts">
import { h, useSlots } from 'vue';
const props = defineProps<{ level: Number }>()
const slot = useSlots()

function render() {
    return h(
        `h${props.level}`,
        slot
    )
}
</script>