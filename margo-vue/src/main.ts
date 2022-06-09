import { Builder, Types } from '../pkg/margo_vue';

async function buildinComponent(builder: Builder) {
    const r = builder.setComponent(Types.File, await import("@/components/File.vue"))
        .setComponent(Types.Block, await import("@/components/Block.vue"))
        .setComponent(Types.BlockCode, await import("@/components/Code.vue"))
        .setComponent(Types.BlockHeading, await import("@/components/Heading.vue"))
        .setComponent(Types.BlockParagraph, await import("@/components/Paragraph.vue"))
        .setComponent(Types.Inline, await import("@/components/Inline.vue"))
        .setComponent(Types.InlineBold, await import("@/components/InlineBold.vue"))
        .setComponent(Types.InlineCode, await import("@/components/InlineCode.vue"))
        .setComponent(Types.InlineItalic, await import("@/components/InlineItalic.vue"))
        .setComponent(Types.InlineLinethrough, await import("@/components/InlineLinethrough.vue"))
        .setComponent(Types.InlineMath, await import("@/components/InlineMath.vue"))
        .setComponent(Types.InlineUnderline, await import("@/components/InlineUnderline.vue"))
        .setComponent(Types.Text, await import("@/components/Text.vue"));
    return r;
}