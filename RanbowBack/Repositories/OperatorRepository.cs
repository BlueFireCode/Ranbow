using MySql.Data.MySqlClient;
using RanbowBack.Config;
using RanbowBack.DbObjects;
using RanbowBack.Enums;
using RanbowBack.Models;
using RanbowBack.Repositories.Base;
using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace RanbowBack.Repositories
{
    public class OperatorRepository : BaseRepository
    {
        public OperatorRepository(string connectionString) : base(connectionString) { }

        public static List<OperatorModel> Operators
        {
            get => _operators.Count < 1 ? new OperatorRepository(Configuration.Instance.Config.ConnectionString).GetAll() : _operators;
        }

        private static List<OperatorModel> _operators = new();

        public List<OperatorModel> GetAll()
        {
            List<DbOperator> dbObjectList = new();

            try
            {
                DbCommand.CommandText = "SELECT * FROM Operator;";
                DbCommand.Parameters.Clear();
                DbConnection.Open();
                using (MySqlDataReader reader = DbCommand.ExecuteReader())
                {
                    while (reader.Read())
                    {
                        dbObjectList.Add(new()
                        {
                            ID = (int)reader["ID"],
                            Name = (string)reader["Name"],
                            IconUrl = Convert.IsDBNull(reader["IconUrl"]) ? null : (string)reader["IconUrl"],
                            ImageUrl = Convert.IsDBNull(reader["ImageUrl"]) ? null : (string)reader["ImageUrl"],
                            Primary1 = (int)reader["Primary1"],
                            Primary2 = Convert.IsDBNull(reader["Primary2"]) ? null : (int?)reader["Primary2"],
                            Primary3 = Convert.IsDBNull(reader["Primary3"]) ? null : (int?)reader["Primary3"],
                            Secondary1 = (int)reader["Secondary1"],
                            Secondary2 = Convert.IsDBNull(reader["Secondary2"]) ? null : (int?)reader["Secondary2"],
                            Secondary3 = Convert.IsDBNull(reader["Secondary3"]) ? null : (int?)reader["Secondary3"],
                            Gadget1 = (int)reader["Gadget1"],
                            Gadget2 = (int)reader["Gadget2"],
#nullable enable
                            Description = Convert.IsDBNull(reader["Description"]) ? null : (string?)reader["Description"],
#nullable disable
                            Attacker = (ulong)reader["Attacker"] != 0
                        });
                    }
                }
                DbConnection.Close();
            }
            catch (Exception e)
            {
                //TODO: Add logging
                Debug.WriteLine(e.Message);
            }
            finally
            {
                if (DbConnection.State == System.Data.ConnectionState.Open)
                {
                    DbConnection.Close();
                }
            }

            if (dbObjectList.Count != 0)
            {
                var weapons = WeaponRepository.Weapons;
                var gadgets = GadgetRepository.Gadgets;
                foreach (var item in dbObjectList)
                {
                    OperatorModel model = new()
                    {
                        ID = item.ID,
                        Name = item.Name,
                        IconUrl = item.IconUrl,
                        ImageUrl = item.ImageUrl,
                        Primary1 = weapons.Find(x => x.ID == item.Primary1),
                        Primary2 = weapons.Find(x => x.ID == item.Primary2),
                        Primary3 = weapons.Find(x => x.ID == item.Primary3),
                        Secondary1 = weapons.Find(x => x.ID == item.Secondary1),
                        Secondary2 = weapons.Find(x => x.ID == item.Secondary2),
                        Secondary3 = weapons.Find(x => x.ID == item.Secondary3),
                        Gadget1 = gadgets.Find(x => x.ID == item.Gadget1),
                        Gadget2 = gadgets.Find(x => x.ID == item.Gadget2),
                        Description = item.Description,
                        Side = item.Attacker ? Side.Attack : Side.Defense
                    };

                    _operators.Add(model);
                }
                return _operators;
            }
            _operators = new();
            return _operators;
        }
    }
}
