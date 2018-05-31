using System;
using System.Collections.Generic;
using System.Linq;
using Oak.OverUnder.Domain.GamePlay;
using Oak.OverUnder.Domain.GamePlay.Models;

namespace Oak.OverUnder.Domain
{
    public class Game
    {
        public GameRound CurrentGameRound { get; private set; }

        public Player Player { get; private set; }

        public Oponent Oponent { get; private set; }

        public Deck Deck { get; private set; }

        public Action<Game> NewCards { get; set; }

        public Action<Game> StatsChanged { get; set; }

        public bool IsDone => Deck.IsEmpty();

        public int TotalRounds { get; set; }

        public Game()
        {
            Player = new Player() { Name = "Player" };
            Oponent = new Oponent() { Name = "Leikjastjóri" };
            Deck = new Deck();
        }

        public void Start()
        {
            // Shuffle
            Deck.Shuffle();            

            while (!Deck.IsEmpty())
            {
                if (CurrentGameRound == null || CurrentGameRound.State != GameRoundState.InProgress)
                {
                    CurrentGameRound = GameRound.Create(Deck.DrawCards().ToList());

                    NewCards?.Invoke(this);

                    switch (CurrentGameRound.State)
                    {
                        case Domain.GamePlay.GameRoundState.PlayerWins:
                            Player.Points++;                            
                            break;
                        case Domain.GamePlay.GameRoundState.OponentWins:
                            Oponent.Points++;                            
                            break;
                    }

                    StatsChanged?.Invoke(this);

                    TotalRounds++;
                }
            }
        }

        public void ChooseUnder()
        {
            CurrentGameRound.ChooseUnder();

            // Leikmaður velur undir, en við svöppum falda spilinu ef það er undir 7
            if ((int)CurrentGameRound.HiddenCard.Number < 7)
            {
                CheatUnder();
                CurrentGameRound.ChooseUnder();
            }
        }

        public void ChooseOver()
        {
            CurrentGameRound.ChooseOver();

            // Leikmaður velur yfir, en við svöppum falda spilinu ef það er yfir 7
            if ((int) CurrentGameRound.HiddenCard.Number > 7)
            {
                CheatOver();
                CurrentGameRound.ChooseOver();
            }
        }

        private void CheatUnder()
        {
            CurrentGameRound.HiddenCard = Deck.HighCards.Pop();
        }

        private void CheatOver()
        {
            CurrentGameRound.HiddenCard = Deck.LowCards.Pop();
        }
 
        public Player GetWinner()
        {
            if (Player.Points == Oponent.Points)
            {
                return Oponent;
            }

            if (Player.Points > Oponent.Points)
            {
                return Player;
            }

            return Oponent;
        }
    }
}
