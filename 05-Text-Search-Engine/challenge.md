# Challenge: Design and Implement a Simple Text Search Engine

## 1. Dataset
The dataset consists of one or multiple files with words separated by space or word boundary characters. You can use this regex to match the words:

```
\w+
```

You can find public resources with large texts. For HTML content, you can strip HTML tags using various methods:
- [Project Gutenberg](https://www.gutenberg.org/): 70,000 free eBooks available for download as plain text files in UTF-8.
- [PizzaChili Texts](https://pizzachili.dcc.uchile.cl//texts.html)
- [Wikipedia Database Download](https://en.wikipedia.org/wiki/Wikipedia:Database_download)
- [Wikimedia Dumps](https://dumps.wikimedia.org/)

## 2. Search Query

### 2.1 Search by Characters
Expose a `search_chars` method that accepts a `term` and `type` parameters. The search scope is each **word** in the dataset, meaning any word (not line) that matches the search criteria is returned as a result. This is like searching for a substring within a word, not necessarily the whole word.

**Limit:** Max 100 results (or a configurable value).

#### **Parameters**
- `term`: A word or part of a word (must not contain spaces or word boundary characters). Acts as a character tokenizer.
- `type`: Defines query mode:
  - **Prefix Search**: Returns all words that start with `term` (e.g., input "abc" returns "abcde").
  - **Suffix Search**: Returns words ending with `term` (e.g., input "xyz" returns "vwxyz").
  - **Contains Search**: Returns words containing `term` anywhere (e.g., input "mid" returns "admidst").

**Bonus:** Implement multiple character group matching with an explanation of logic and behavior.

### 2.2 Search by Words
Expose a `search_words` method that accepts a `term` and `type` parameters. The search scope is each **line** in the dataset, meaning any line that matches the search criteria is returned as a result. Matching is done against words in the line.

#### **Parameters**
- `term`: One or multiple words separated by spaces or word boundary characters. Words should be split and treated as tokens.
- `type`: Defines query mode:
  - **Prefix Search**: Returns lines starting with the given word sequence (e.g., "abc ba" matches "abc ba de").
  - **Suffix Search**: Returns lines ending with the given word sequence (e.g., "wt xyz" matches "st wt xyz").
  - **Contains Search**: Returns lines containing the word sequence anywhere (e.g., "mad dog" matches "oho mad dog is here").

**Tip:** The structure is similar to `search_chars`, but operates at the line level instead of word level.

**Bonus:** Implement wildcard support for `*` and `?` in queries.

## 3. Relevance Ranking
Sort the results by relevance to the query term using the **Levenshtein distance** to prioritize exact matches at the top.

## 4. Constraints
- Each **word** can have a maximum of **255 characters**.
- Each **line** can have a maximum of **32,768 characters** (e.g., Jonathan Coe's *The Rotters' Club* contains a 13,955-word sentence).
- The dataset may include any **Unicode characters**, so ensure **UTF-8** encoding is used.
- The solution must efficiently handle **millions of words**.

## 5. Restrictions
This project is designed to help you understand how search engines work. Avoid libraries that provide built-in text search solutions (e.g., Apache Lucene). However, you may use libraries for:
- **Levenshtein distance calculations**
- **Tree structures**
- **Word tokenization using regex**

## 6. Evaluation
Your solution will be evaluated on:
- **Efficiency**
- **Accuracy**
- **Scalability** to handle large datasets
- **User experience**

Aim to build a well-structured and efficient text search system that meets the project requirements.

## Submission
Submit your solution in a **public repository** and send the link to **coding-challenge.why724@passfwd.com** by **Feb 6 at 19:00 GMT+2**.

A follow-up discussion will take place at [Star Tech R&D Reloaded](https://www.meetup.com/star-tech-rd-reloaded), so ensure you are a member to receive notifications.

## Prize
The community will vote for the best solution, and the **$42 prize** includes:
- A book (~$20) for the winner
- The remaining amount donated to the **Free and Open-Source Software (FOSS) community**

## Resources & Hints
- [Trie](https://en.wikipedia.org/wiki/Trie)
- [Suffix Tree](https://en.m.wikipedia.org/wiki/Suffix_tree)
- [N-gram & Overlap](https://en.wikipedia.org/wiki/N-gram)
- Implement a **hashing function** with smaller output
- Use **string pooling** to reduce storage

## Fun Facts
**Longest Words:**
- **Māori:** *Taumatawhakatangihangakoauauotamateaturipukakapikimaungahoronukupokaiwhenuakitanatah* (85 characters)
- **German:** *Donaudampfschiffahrtselektrizitätenhauptbetriebswerkbauunterbeamtengesellschaft* (79 characters)
- **English:** *Pneumonoultramicroscopicsilicovolcanoconiosis* (45 characters) - A lung disease caused by inhaling fine silica dust.
- **Sanskrit literature** includes words exceeding **100 characters** in transliteration.

---

**Let the coding begin!** 🚀

