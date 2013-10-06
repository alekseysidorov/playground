#include <stdio.h>
#include <stdlib.h>

struct list_node
{
	int value;
	struct list_node *next;
};
struct list_node;
typedef struct list_node list_node;

inline list_node *list_node_end(list_node *head)
{
	while(head->next)
		head = head->next;
	return head;
}

inline list_node *list_node_create(int value, list_node *next)
{
	list_node *item = (list_node*)malloc(sizeof(list_node));
	if (item != NULL) {
		item->value = value;
		item->next = next;
		return item;
	}
	return NULL;
}

inline void list_node_append(list_node *head, int value)
{
	list_node *node = list_node_create(value, NULL);
	list_node *tail = list_node_end(head);
	tail->next = node;
}

inline void list_node_prepend(list_node **head, int value)
{
	list_node *item = list_node_create(value, head[0]);
	head[0] = item;
}

inline list_node *list_node_item_at(list_node *head, int index)
{
	int i = 0;
	while (head->next != NULL && i < index) {
		i++;
		head = head->next;
	}	
	if (i == index && head != NULL)
		return head;
	return NULL;
}

inline void list_node_insert(list_node *head, int index, int value)
{
	head = list_node_item_at(head, index);
	if (head != NULL) {
		list_node *tmp = head->next;
		head->next = list_node_create(value, tmp);
	}
}

void list_node_bubble_sort(list_node *head)
{
	list_node *i = head;
	while (i->next != NULL) {
		list_node *j = head;
		while (j->next != NULL) {
			if (j->value > j->next->value) {
				j->value += j->next->value;
				j->next->value = j->value - j->next->value;
				j->value = j->value - j->next->value;
			}
			j = j->next;
		}
		i = i->next;
	}
}

int main() 
{
	list_node *head = list_node_create(10, NULL);
	list_node_append(head, 20);
	list_node_append(head, 15);
	list_node_prepend(&head, 5);
	list_node_append(head, 18);
	list_node_insert(head, 1, 666);

	list_node *tmp = head;
	list_node_bubble_sort(tmp);
	while (tmp) {
		printf("%d ", tmp->value);
		tmp = tmp->next;
	}

	int a[5];
	a[4] = 100500;
	printf("\n olololl = %d ", *(a + 4));

	return 0;
}
