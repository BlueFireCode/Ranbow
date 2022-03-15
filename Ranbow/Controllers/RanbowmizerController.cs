using Microsoft.AspNetCore.Mvc;
using Ranbow.ViewModels;
using RanbowBack.Enums;
using RanbowBack.Repositories;
using Ranbowmizer.Operators;
using System.Diagnostics;
using System.Linq;

namespace Ranbow.Controllers
{
    public class RanbowmizerController : Controller
    {
        public IActionResult Ranbow()
        {
            var list = OperatorRepository.Operators;
            if (list is null)
            {
                return Error();
            }

            ViewBag.Operators = list;

            return View();
        }

        public IActionResult Attacker()
        {
            var list = OperatorRepository.Operators;
            if (list is null)
            {
                return Error();
            }

            ViewBag.Random = Randomize.RandomizeOperator(list.Where(x => x.Side == Side.Attack).ToList());

            return View();
        }

        public IActionResult Defender()
        {
            var list = OperatorRepository.Operators;
            if (list is null)
            {
                return Error();
            }

            ViewBag.Random = Randomize.RandomizeOperator(list.Where(x => x.Side == Side.Defense).ToList());

            return View();
        }

        [ResponseCache(Duration = 0, Location = ResponseCacheLocation.None, NoStore = true)]
        public IActionResult Error()
        {
            return View(new ErrorViewModel { RequestId = Activity.Current?.Id ?? HttpContext.TraceIdentifier });
        }
    }
}
