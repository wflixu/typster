<template>
    <div class="table-of-contents">
        <div class="empty-state" v-if="items.length === 0">
            <p>Start editing your document to see the outline.</p>
        </div>
        <template v-else>
            <template v-for="(item, i) in items" :key="item.id">

                <div :class="{
                    'is-active': item.isActive && !item.isScrolledOver,
                    'is-scrolled-over': item.isScrolledOver,
                }" :style="{ '--level': item.level }" :item="item" :index="i + 1">
                    <a :href="'#' + item.id" @click.prevent="(event) => onItemClick(event, item.id)"
                        :data-item-index="item.itemIndex">
                        {{ item.textContent }}
                    </a>
                </div>
            </template>
        </template>
    </div>
</template>

<script setup lang="ts">
import { TextSelection } from '@tiptap/pm/state';
import { useSystemStoreHook } from '../../store/store';


defineProps<{
    items: any[]
}>()

const { editor } = useSystemStoreHook();

const onItemClick = (e, id) => {
    if (editor.value) {
        const element = editor.value.view.dom.querySelector(`[data-toc-id="${id}"`)
        const pos = editor.value.view.posAtDOM(element, 0)

        // set focus
        const tr = editor.value.view.state.tr

        tr.setSelection(new TextSelection(tr.doc.resolve(pos)))

        editor.value.view.dispatch(tr)

        editor.value.view.focus()

        // eslint-disable-next-line
        if (history.pushState) {
            // eslint-disable-next-line
            history.pushState(null, null, `#${id}`)
        }

        window.scrollTo({
            top: element.getBoundingClientRect().top + window.scrollY,
            behavior: 'smooth',
        })
    }
};
</script>

<style scoped>
.table-of-contents {
    display: flex;
    flex-direction: column;
    font-size: 0.875rem;
    gap: 0.25rem;
    overflow: auto;
    text-decoration: none;

    >div {
        border-radius: 0.25rem;
        padding-left: calc(0.875rem * (var(--level) - 1));
        transition: all 0.2s cubic-bezier(0.65, 0.05, 0.36, 1);

        &:hover {
            background-color: var(--color-bg-secondary);
        }
    }

    .empty-state {
        color: var(--color-fg-tertiary);
        user-select: none;
    }

    .is-active a {
        color: var(--color-accent-primary);
    }

    .is-scrolled-over a {
        color: var(--color-fg-tertiary);
    }

    a {
        color: var(--color-fg-primary);
        display: flex;
        gap: 0.25rem;
        text-decoration: none;

        &::before {
            content: attr(data-item-index) '.';
        }
    }
}
</style>