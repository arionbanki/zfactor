using System;
using System.Collections.Generic;
using System.Linq;
using System.Security.Cryptography;

namespace Oak.OverUnder.Domain.GamePlay.Models
{
    public class Deck
    {
        public Stack<Card> Cards { get; set; }

        public Stack<Card> HighCards { get; set; }

        public Stack<Card> LowCards { get; set; }

        public Stack<Card> UnusedCards { get; set; }

        public Enums.Back Back
        {
            get;
            set;
        }

        public Deck()
        {
            LoadCards();
        }


        public bool IsEmpty()
        {
            return !Cards.Any();
        }

        public IList<Card> DrawCards()
        {
            return new List<Card> { Cards.Pop(), Cards.Pop() };
        }

        internal void Shuffle()
        {
            var shuffledCards = new Stack<Card>();
            RNGCryptoServiceProvider rnd = new RNGCryptoServiceProvider();
            var index = 0;
            foreach (var card in Cards.OrderBy(x => GetNextInt32(rnd)))
            {
                card.Position = index;
                shuffledCards.Push(card);
                index++;
            }

            Cards = shuffledCards;
        }

        private int GetNextInt32(RNGCryptoServiceProvider rnd)
        {
            byte[] randomInt = new byte[4];
            rnd.GetBytes(randomInt);
            return Convert.ToInt32(randomInt[0]);
        }

        private void LoadCards()
        {
            LowCards = new Stack<Card>();
            HighCards = new Stack<Card>();
            UnusedCards = new Stack<Card>();
            Cards = new Stack<Card>();
            for (var i = 0; i < Enum.GetValues(typeof(Enums.Suit)).Length; i++)
            {
                for (var j = 0; j < Enum.GetValues(typeof(Enums.Number)).Length; j++)
                {
                    if ((Enums.Number)j != Enums.Number._JOKER)
                    {
                        var card = new Card();
                        card.Suit = (Enums.Suit)i;
                        card.Number = (Enums.Number)j;
                        card.Back = this.Back;

                        if ((Enums.Number) j == Enums.Number._A || (Enums.Number) j == Enums.Number._K || (Enums.Number)j == Enums.Number._Q)
                        {
                            HighCards.Push(card);
                        }

                        else if ((Enums.Number)j == Enums.Number._2 || (Enums.Number)j == Enums.Number._3 || (Enums.Number)j == Enums.Number._4)
                        {
                            LowCards.Push(card);
                        }
                        else
                        {
                            if (Cards.Count < 26)
                            {
                                Cards.Push(card);
                            }
                            else
                            {
                                UnusedCards.Push(card);
                            }
                        }
                    }
                }
            }
        }
    }
}
