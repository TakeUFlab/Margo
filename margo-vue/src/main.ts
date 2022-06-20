import init, { Builder, Types } from 'margo-wasm-vue'
import File from "@/components/File.vue"
import Block from "@/components/Block.vue"
import Code from "@/components/Code.vue"
import Heading from "@/components/Heading.vue"
import Paragraph from "@/components/Paragraph.vue"
import Inline from "@/components/Inline.vue"
import InlineBold from "@/components/InlineBold.vue"
import InlineCode from "@/components/InlineCode.vue"
import InlineItalic from "@/components/InlineItalic.vue"
import InlineLinethrough from "@/components/InlineLinethrough.vue"
import InlineMath from "@/components/InlineMath.vue"
import InlineUnderline from "@/components/InlineUnderline.vue"
import Text from "@/components/Text.vue"

function buildinComponent(builder: Builder) {
    const r = builder.setComponent(Types.File, File)
        .setComponent(Types.Block, Block)
        .setComponent(Types.BlockCode, Code)
        .setComponent(Types.BlockHeading, Heading)
        .setComponent(Types.BlockParagraph, Paragraph)
        .setComponent(Types.Inline, Inline)
        .setComponent(Types.InlineBold, InlineBold)
        .setComponent(Types.InlineCode, InlineCode)
        .setComponent(Types.InlineItalic, InlineItalic)
        .setComponent(Types.InlineLinethrough, InlineLinethrough)
        .setComponent(Types.InlineMath, InlineMath)
        .setComponent(Types.InlineUnderline, InlineUnderline)
        .setComponent(Types.Text, Text)
    return r
}

export default init

export {
    buildinComponent,
    Builder,
    Types,
    File,
    Block,
    Code,
    Heading,
    Paragraph,
    Inline,
    InlineBold,
    InlineCode,
    InlineItalic,
    InlineLinethrough,
    InlineMath,
    InlineUnderline,
    Text,
}