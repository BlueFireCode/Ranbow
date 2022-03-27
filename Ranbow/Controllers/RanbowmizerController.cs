using Microsoft.AspNetCore.Mvc;
using Ranbow.ViewModels;
using RanbowBack.Enums;
using RanbowBack.Models;
using RanbowBack.Repositories;
using Ranbowmizer.Operators;
using System.Collections.Generic;
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

            return View(new MainModel()
            {
                Attackers = list.Where(x => x.Side == Side.Attack).ToList(),
                Defenders = list.Where(x => x.Side == Side.Defense).ToList()
            });
        }

        [HttpPost]
        public IActionResult Attacker(MainModel model)
        {
            var list = OperatorRepository.Operators.Where(x =>
            {
                var op = model.Attackers.FirstOrDefault(y => y.ID == x.ID);
                return op is not null && op.IsSelected;
            });
            if (list is null)
            {
                return Error();
            }

            ViewBag.Random = Randomize.RandomizeOperator(list.Where(x => x.Side == Side.Attack && x.IsSelected).ToList(), model.GonneSixDisabled);

            return View(model);
        }

        [HttpPost]
        public IActionResult Defender(MainModel model)
        {
            var list = OperatorRepository.Operators.Where(x =>
            {
                var op = model.Defenders.FirstOrDefault(y => y.ID == x.ID);
                return op is not null && op.IsSelected;
            });
            if (list is null)
            {
                return Error();
            }

            ViewBag.Random = Randomize.RandomizeOperator(list.Where(x => x.Side == Side.Defense && x.IsSelected).ToList(), model.GonneSixDisabled);

            return View(model);
        }

        [ResponseCache(Duration = 0, Location = ResponseCacheLocation.None, NoStore = true)]
        public IActionResult Error()
        {
            return View(new ErrorViewModel { RequestId = Activity.Current?.Id ?? HttpContext.TraceIdentifier });
        }
    }

    public class MainModel
    {
        public List<OperatorModel> Attackers { get; set; }
        public List<OperatorModel> Defenders { get; set; }
        public bool GonneSixDisabled { get; set; } = true;
    }
}
