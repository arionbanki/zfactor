using System;
using System.Linq;
using Oak.OverUnder.Domain;

namespace Oak.OverUnder.Console
{
    class Program
    {
        protected Program()
        {

        }

        static void Main(string[] args)
        {
            var game = new OverUnder.Domain.Game();
            game.NewCards += NewCards;
            game.StatsChanged += StatsChanged;
            game.Start();
            System.Console.WriteLine("Velkominn");
        }

        private static void NewCards(Game game)
        {
            System.Console.WriteLine($"Spil í borði er: {game.CurrentGameRound.VisibleCard.Number} {game.CurrentGameRound.VisibleCard.Suit}");
            System.Console.WriteLine("Veldu 1 fyrir undir og 2 fyrir yfir: ");
            var input = System.Console.ReadKey();

            switch (input.Key) //Switch on Key enum
            {
                case ConsoleKey.D1:
                    game.ChooseUnder();
                    break;
                case ConsoleKey.D2:
                    game.ChooseOver();
                    break;
            }

            System.Console.WriteLine($"Falda spil var: {game.CurrentGameRound.HiddenCard.Number} {game.CurrentGameRound.HiddenCard.Suit}");
        }

        private static void StatsChanged(Game game)
        {
            System.Console.WriteLine($"Cards left:{game.Deck.Cards.Count}");
        
            switch (game.CurrentGameRound.State)
            {
                case Domain.GamePlay.GameRoundState.PlayerWins:
                    System.Console.WriteLine("Player wins");
                    break;
                case Domain.GamePlay.GameRoundState.OponentWins:
                    System.Console.WriteLine("Oponent wins");
                    break;
            }

            if (game.IsDone)
            {
                System.Console.WriteLine("Done!!");
                System.Console.WriteLine($"Player total:{game.Player.Points}");
                System.Console.WriteLine($"Oponent total:{game.Oponent.Points}");

                if (game.GetWinner() == null)
                {
                    System.Console.WriteLine("Jafntefli");
                }
                else
                {
                    System.Console.WriteLine($"Winner is: {game.GetWinner().Name}");
                }

                System.Console.WriteLine("Smelltu á einhvern takka til að hætta");
                System.Console.ReadLine();
            }
        }
    }
}
