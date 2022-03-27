using MySql.Data.MySqlClient;
using RanbowBack.Config;
using RanbowBack.DbObjects;
using RanbowBack.Models;
using RanbowBack.Repositories.Base;
using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace RanbowBack.Repositories
{
    public class GadgetRepository : BaseRepository
    {
        public GadgetRepository(string connectionString) : base(connectionString) { }

        public static List<GadgetModel> Gadgets
        {
            get => _gadgets.Count < 1 ? new GadgetRepository(Configuration.Instance.Config.ConnectionString).GetAll() : _gadgets;
        }
        private static List<GadgetModel> _gadgets = new();

        public List<GadgetModel> GetAll()
        {
            List<DbGadget> dbObjectList = new();

            try
            {
                DbCommand.CommandText = "SELECT * FROM Gadget;";
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
                            IconUrl = Convert.IsDBNull(reader["IconUrl"]) ? null : (string)reader["IconUrl"]
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
                foreach (var item in dbObjectList)
                {
                    _gadgets.Add(new()
                    {
                        ID = item.ID,
                        Name = item.Name,
                        IconUrl = item.IconUrl
                    });
                }
                return _gadgets;
            }
            _gadgets = new();
            return _gadgets;
        }

        public bool Read(int id, out GadgetModel entity)
        {
            bool result = false;
            DbGadget dbitem = null;

            try
            {
                DbCommand.CommandText = "SELECT * FROM Gadget WHERE ID = @id;";
                DbCommand.Parameters.Clear();
                DbCommand.Parameters.AddWithValue("id", id);
                DbConnection.Open();
                using (MySqlDataReader reader = DbCommand.ExecuteReader())
                {
                    reader.Read();
                    dbitem = new()
                    {
                        ID = id,
                        Name = (string)reader["Name"],
                        IconUrl = Convert.IsDBNull(reader["IconUrl"]) ? null : (string)reader["IconUrl"]
                    };
                }
                DbConnection.Close();

                result = true;
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

            if (result && dbitem is not null)
            {
                entity = new()
                {
                    ID = dbitem.ID,
                    Name = dbitem.Name,
                    IconUrl = dbitem.IconUrl
                };
                return result;
            }
            entity = null;
            return false;
        }
    }
}
