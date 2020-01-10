//
// Created by waff on 12/20/19.
//

#include "desktop_entry.h"

size_t deb_push(desktop_entry_batch* batch, desktop_entry* entry) {
    desktop_entry_batch_node* node = malloc(sizeof(desktop_entry_batch_node));
    node->entry = entry;
    node->next = NULL;
    node->prev = NULL;

    if (batch->length == 0) {
        batch->first = node;
        batch->last = node;
        return ++batch->length;
    }

    batch->last->next = node;
    node->prev = batch->last;
    batch->last = node;

    return ++batch->length;
}

size_t deb_unshift(desktop_entry_batch* batch, desktop_entry* entry) {
    desktop_entry_batch_node* node = malloc(sizeof(desktop_entry_batch_node));
    node->entry = entry;
    node->next = NULL;
    node->prev = NULL;

    if (batch->length == 0) {
        batch->first = node;
        batch->last = node;
        return ++batch->length;
    }

    node->next = batch->first;
    batch->first->prev = node;
    batch->first = node;

    return ++batch->length;
}

desktop_entry* deb_pop(desktop_entry_batch* batch) {
    if (batch->length == 0) return NULL;

    desktop_entry_batch_node* node = batch->last;
    batch->last = node->prev;
    node->prev->next = NULL;
    node->prev = NULL;

    if (--batch->length == 0) {
        batch->first = NULL;
    }

    desktop_entry* entry = node->entry;

    free(node);
    return entry;
}

desktop_entry* deb_shift(desktop_entry_batch* batch) {
    if (batch->length == 0) return NULL;

    desktop_entry_batch_node* node = batch->first;
    batch->first = node->next;
    node->next->prev = NULL;
    node->next = NULL;

    if (--batch->length == 0) {
        batch->last = NULL;
    }

    desktop_entry* entry = node->entry;

    free(node);
    return entry;
}

void deb_concat(desktop_entry_batch* target, desktop_entry_batch* source) {
    if (target == NULL) {
        fprintf(stderr, "[ERROR] target is NULL");
        return;
    }

    if (source == NULL) {
        return;
    };

    if (source->length == 0) {
        deb_destructor(source);
        return;
    }

    if (target->length == 0) {
        target->first = source->first;
        target->last = source->last;
        target->length = source->length;

        source->first = NULL;
        source->last = NULL;
        deb_destructor(source);
        return;
    }

    source->first->prev = target->last;
    target->last->next = source->first;
    target->last = source->last;

    target->length += source->length;

    source->first = NULL;
    source->last = NULL;
    deb_destructor(source);
}

desktop_entry_batch* deb_constructor() {
    desktop_entry_batch* batch = malloc(sizeof(desktop_entry_batch));

    batch->first = NULL;
    batch->last = NULL;
    batch->length = 0;

    return batch;
}

void deb_destructor(desktop_entry_batch* batch) {
    if (batch->length) {
        desktop_entry_batch_node* curr = batch->first;

        while (curr != NULL) {
            free(curr->entry);

            desktop_entry_batch_node* next = curr->next;
            free(curr);

            curr = next;
        }

    }

    free(batch);
}

void deb_destructor_no_entry(desktop_entry_batch* batch) {
    if (batch->length) {
        desktop_entry_batch_node* curr = batch->first;

        while (curr != NULL) {
            desktop_entry_batch_node* next = curr->next;
            free(curr);

            curr = next;
        }

    }

    free(batch);
}

desktop_entry* de_constructor(const char* gtk_launch_name) {
    desktop_entry* entry = malloc(sizeof(desktop_entry));
    strcpy(entry->gtk_launch_name, gtk_launch_name);

    return entry;
}
