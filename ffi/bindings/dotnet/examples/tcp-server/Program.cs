﻿using System;
using System.Collections.Generic;
using System.Threading;
using rodbus;

namespace example
{
    class Program
    {
        class LogHandler : ILogHandler
        {
            public void OnMessage(LogLevel level, string message)
            {
                Console.WriteLine($"{level} - {message}");
            }
        }


        class WriteHandler : IWriteHandler
        {
            public WriteResult WriteMultipleCoils(ushort start, ICollection<Bit> it, Database database)
            {
                foreach (var bit in it)
                {
                    if (!database.UpdateCoil(bit.Index, bit.Value))
                    {
                        return WriteResult.CreateException(rodbus.Exception.IllegalDataAddress);
                    }
                }

                return WriteResult.CreateSuccess();
            }

            public WriteResult WriteMultipleRegisters(ushort start, ICollection<Register> it, Database database)
            {
                foreach (var bit in it)
                {
                    if (!database.UpdateHoldingRegister(bit.Index, bit.Value))
                    {
                        return WriteResult.CreateException(rodbus.Exception.IllegalDataAddress);
                    }
                }

                return WriteResult.CreateSuccess();
            }

            public WriteResult WriteSingleCoil(bool value, ushort index, Database database)
            {
                if (database.UpdateCoil(index, value))
                {
                    return WriteResult.CreateSuccess();
                }
                else
                {
                    return WriteResult.CreateException(rodbus.Exception.IllegalDataAddress);
                }
            }

            public WriteResult WriteSingleRegister(ushort value, ushort index, Database database)
            {
                if (database.UpdateHoldingRegister(index, value))
                {
                    return WriteResult.CreateSuccess();
                }
                else
                {
                    return WriteResult.CreateException(rodbus.Exception.IllegalDataAddress);
                }
            }
        }

        class DatabaseUpdate : IDatabaseCallback
        {
            readonly Action<Database> action;
            public DatabaseUpdate(Action<Database> action)
            {
                this.action = action;
            }

            public void Callback(Database database)
            {
                this.action.Invoke(database);
            }
        }

        static void Main(string[] args)
        {
            Logging.SetMaxLogLevel(LogLevel.Info);
            Logging.SetHandler(new LogHandler());

            using var runtime = new Runtime(new RuntimeConfig { NumCoreThreads = 1 });
            var map = new DeviceMap();
            map.AddEndpoint(1, new WriteHandler(), new DatabaseUpdate((db) =>
            {
                for (ushort i = 0; i < 10; ++i)
                {
                    db.AddCoil(i, false);
                    db.AddDiscreteInput(i, false);
                    db.AddHoldingRegister(i, 0);
                    db.AddInputRegister(i, 0);
                }
            }));

            var server = Server.CreateTcpServer(runtime, "127.0.0.1:502", 10, map);

            ushort registerValue = 0;
            bool bitValue = false;

            while (true)
            {
                server.Update(1, new DatabaseUpdate((db) =>
                {
                    registerValue += 1;
                    bitValue = !bitValue;
                    for (ushort i = 0; i < 10; ++i)
                    {
                        db.UpdateDiscreteInput(i, bitValue);
                        db.UpdateInputRegister(i, registerValue);
                    }
                }));
                Thread.Sleep(1000);
            }
        }
    }
}
