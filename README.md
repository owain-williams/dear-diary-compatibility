# Dear Diary - Match Maker
This piece of software was inspired by a childhood toy my partner found in her Mum's attic, called Dear Diary. It's a personal organiser type affair from the 90's, in hot pink, and is aimed at young girls. It has features like an address book, a fortune teller, and a journal. 

One of said features is a "Match Maker", where you tell it your birthday and then your crush's birthday, and it will give you a surprisingly dark piece of advice, like "Make it a habit - give small presents to your partner" or "Do not show a sad face - it will make your partner also sad".

The "Dear Diary" actually gave a number, and you had to have the "Match Maker Hint Table" card that had a number -> advice lookup like "12 = It is in your interest - to make your partner happier".

I copied the advice 1:1 into this software, and created a seeded random number from the sum of both date of births (so if you swapped the DOBs you would get the same seed), in a range, and return the advice for that number.